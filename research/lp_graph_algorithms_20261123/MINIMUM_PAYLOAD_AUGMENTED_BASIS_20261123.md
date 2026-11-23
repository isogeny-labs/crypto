# Minimum source-preserving augmented-row payload via matroid greedy — 2026-11-23

Status: preregistered mathematical and native-test protocol; **not executed or measured at registration**. Lane [draft PR #5043](https://github.com/isogeny-labs/crypto/pull/5043), source parent `fa71b46b9225f811e7e45ae42fdcc84cdd9a806e`. No merge, paid compute, outside-lane worktree edits, or leaderboard promotion.

## Question and proposed mechanism

The current exact projected-rank selector keeps the **first** original independent augmented rows. Input order can select costly large rows despite cheaper source-bearing rows spanning the same finite-field system. For every canonical LP-free original `CompleteRelation` row `r` define **fixed nonnegative additive payload weight** `w(r)=len(r.small)+len(r.sources)` (count entries, not bytes or solver fill). Sort original row indices by `(w(r), original_index)`, then perform the existing exact sparse elimination **on references in that order**, carrying original indices and retaining at most one known-RHS contradiction. Copy only selected **unchanged original rows** sorted by original input index. Do not sort/mutate the original stream or reconstruct source witnesses.

The original augmented row vectors `[S_i | b_i]` represent a linear matroid over the **prime subgroup-order field F_ell**. A row is independent of previously kept rows iff its small projection has a fresh pivot, OR its reduced small projection is zero with a nonzero residual known RHS and no earlier such residual was retained. Greedy by nondecreasing fixed row weight therefore produces a **minimum total `w` original-row basis of rowspan([S|b])** (standard matroid exchange argument). A contradictory RHS creates a rank-one last-column direction, which must be retained explicitly and must cause solver refusal if all unknowns are modeled. The theorem does NOT minimize fill, peak working memory, wall time, scalar-product cost, total ECDLP cost, or asymptotic complexity. It does NOT hold for a hidden target coefficient in a purportedly known RHS.

## Frozen comparison

Control: current `projected_rank::select_augmented_basis` (input-order exact echelon); candidate: additive opt-in `select_min_payload_augmented_basis` using the **same source-preserving original-Rust echelon** and one deterministic index sort. The default `audit_projected_rank`, evaluator, CLI, old JSON and production solver must remain unchanged.

Test exact fixed duplicate/heavy-versus-light examples with RHS contradiction, explicit protected `u32::MAX` column and source identities. Run >=128 deterministic canonical synthetic streams over `F_3,F_7,F_101,F_{2147446831}`, using an independent dense original-matrix small/augmented rank oracle and verifying source preservation, exact retained rank, ordered original indices, and `candidate_payload<=control_payload`. For >=32 small (<=8 rows) instances, enumerate all original-row subsets and verify candidate equals the true minimum additive payload among all augmented-rank bases, with deterministic original-index tie break only when needed. Reject noncanonical inputs and noninvertible pivots atomically. Do not confuse algebra-only Prime31-`ell` fixtures with actual `p=2^31-1` group relations.

Record source revision, tests and counters. Charge `O(R log R)` key sort, `O(R)` index storage, full exact sparse echelon, ordered-map work and selected sparse/source clones; expose sort comparisons if available. A payload decrease alone is **not** a runtime improvement. Candidate is falsified by any incorrect rank/provenance/contradiction handling, payload worse than control under the fixed weight, or failed exhaustive optimality fixture.

## Authorization and sources

Execution only in existing source-exact, free hosted graph native test workflow after the final source revision; no competing benchmarks, paid jobs or duplicate runs. Main root-crate build still has an out-of-lane parser failure; leave that file alone. No authentic group-verified ordinary Prime31 two-LP stream is identified; do not replace it with synthetic data for whole-solver claims.

Source/coordination reads: current `AGENTS.md`, `docs/ACTIVE.md`, graph catalog `README.md`/`SOURCES.md`, scoped log and current #5043, Rust `large_prime_graph/projected_rank.rs`, source-exact native tests and current `cryptanalysis` gain-forest/`crypto-autoresearcher` search results. Primary background: Cavallar (2000) https://ir.cwi.nl/pub/4456 (sparse filtering weight), Diem (2006) https://arxiv.org/abs/math/0606607 (double-LP recombination). This finite-field matroid consequence has **no novelty claim**.

KB receipt: research-KB and Conductor/agent-worker tools absent from available runtime registry, repository wrapper unavailable without a worktree; explicit bounded original-source and primary-literature fallback. One coordinator only; no parallel worker result claimed.
