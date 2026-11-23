# Independent raw-matrix projection-rank identity: native Rust qualification

Date: 2026-11-23 UTC. Owner: graph-IC draft PR #4988, disjoint from solver uniqueness PR #4972 and gram-preconditioning PR #4960. Status at registration: mathematical derivation and test protocol only; no new test receipt. [pcc:unverified]

## Hypothesis, exact control and falsification

Let L in F_ell^(R x C) contain the normalized large-prime coefficients and S in F_ell^(R x B) contain normalized original factor-base coefficients. Let b be the **known-constant** right-hand-side column; all unknown logs, including the target log, must occur among S's columns. Let W have rows forming an exact basis of ker(L^T), supplied by the existing two-LP forest. Then

    rank(W S)       = rank([L | S])     - rank(L)
    rank([W S | Wb]) = rank([L | S | b]) - rank(L).

Proof: left multiplication by an invertible R x R basis-change matrix whose first rank(L) rows span a row complement to ker(L^T) and whose final R-rank(L) rows form W takes L to a block with full-row-rank upper block and zero lower block. Eliminate upper-block S and b contributions using the L columns; the rank increment is the lower block's rank. This is exact finite-field linear algebra, not a graph-cycle bound or an ECDLP speedup.

**Independent oracle:** calculate the three dense ranks DIRECTLY from the original canonical LP matrix and original small payloads (not from forest witnesses), with small-column IDs compressed by a bijection (including sparse high target-column IDs). Compare the two differences to `audit_projected_rank` on all replayed forest witnesses. The oracle never consumes forest source coefficients and therefore catches a wrong witness basis or projected-rank implementation that the existing witness-only cross-check might miss.

**Control / experiment:** native `tests/large_prime_graph.rs` deterministic seeded canonical arity<=2 streams, F3/F7/F101/F1000003 and subgroup-order F2147446831 (prime), 2..35 rows, <=12 LP identities and <=18 small-column identities including u32::MAX target column; test raw duplicate small terms, cancellations, one empty-LP row and unbalanced cycles, plus a fixed rank-zero/fixed-RHS contradiction. With generous fixed limits, replay all forest, pivot, and MST output witnesses and compare each arm's useful rank and augmented rank against the raw oracle. Verify strict LP rank, nullity, and forest witness count. If MST refuses under a deliberately tight fixed row cap, require atomic None and charge/reuse the pivot fallback; never count a partial output.

**Falsification:** any forest useful/augmented rank disagreement with the raw identity; any source replay failure; any pivot/MST useful or augmented rank exceeding the raw bound; any refusal exposing partial relations; or any noncanonical result admitted by the rank auditor. No timing-derived speed conclusion.

## Resource and execution envelope

Existing `graph-ic-gain-native.yml` GitHub-hosted exact-source Rust harness only, release mode, no paid nodes, no independent worker/extra build and no performance panel; roughly 128 deterministic small systems and one fixed counterexample. Existing CI jobs must be checked before declaring a new result. Do not run an authentic Prime31 benchmark or a full cold solver based on the oracle result alone; the natural group-verified source stream and full crate gate remain separate. Counts report tested scopes only.

## Sources checked

- AGENTS.md, docs/ACTIVE.md, docs/session-logs/2026-11-23.md, graph catalog and SOURCES.md; current PR #4988 projected_rank.rs, witness.rs, and the original pivot/MST filters. Related lanes #4972 target uniqueness and #4960 Gram preconditioning are separate.
- Bounded prior-work review: isogeny-labs/cryptanalysis experiments/ic-assumption-graph and experiments/ic-bench/prime_bridge.py; isogeny-labs/crypto-autoresearcher 2026-11-21 IC tranche planning. These are receipts previously pinned in PROJECTED_USEFUL_RANK_20261123.md; no new claim about absence.
- Primary papers: Diem (2006) https://arxiv.org/abs/math/0606607 and Cavallar (2000) https://ir.cwi.nl/pub/4456. They motivate relation filtering and rank accounting, not this experiment's empirical outcome or a prime-field elliptic speedup.
- Sources checked (KB): research-kb MCP tools absent from this runtime; `scripts/research-kb` wrapper cannot run without the sibling checkout/executable. Bounded repository-source and primary-paper fallback above. No KB source_id is claimed; conductor/work reservations and independent worker spawning are not available. Existing PR #4988 remains the non-overlapping owner; no other worktree is modified.

## Evidence ledger

- Registered before authoring the native test; no new correctness or performance result yet.

## Implemented native test and evidence (2026-11-23 15:47 UTC)

The exact source/test commit `a15e0cd0ad1f0c653ebe24fbe31f2c397ffa59ba` was checked by hosted [Actions #45208649527](https://github.com/isogeny-labs/crypto/actions/runs/45208649527), job `137811171128`: **35/35 original-module Rust 1.100.0 release tests passed**, including `raw_block_rank_identity_independently_checks_forest_pivot_mst`. This test exercised 128 seeded arity<=2 streams over five primes (including subgroup-order ell=2147446831), raw repeated/canceling small terms, LP-free rows, protected column u32::MAX, replay for all three arms and deliberate atomic MST refusal with charged pivot fallback. Exact rank([L|S])-rank(L) and rank([L|S|b])-rank(L) matched forest's sparse projected-rank and augmented-rank audits on all 128 systems. Pivot and MST/fallback ranks did not exceed the raw oracle. A fixed three-row F7 contradiction had LP rank 1, zero useful small rank and one augmented-rank increment. No natural elliptic-curve input is claimed.

Original source hashes reported by the run: `large_prime_graph.rs` SHA256 `48070937d7c7a6d046c545438c1a40794bec5d544c63f075f6bd48b35ec8d597`, `witness.rs` `80dfe96e62779b661ca5a28a593ac91c65b23ec022d90f9a996e48d92b304fdc`, `projected_rank.rs` `9c7c2cef51323ce9be930fdf4c8142002d13c55902b9c642eabf47c3747a1fec`, and test `large_prime_graph.rs` `cbd67d310e35276322f64d636a850f04590165c33c7b6cfd8addfeb06cd9b78c`. The exact checkout was the workflow-generated merge commit `107d3ece2857b4f8019958cd0872bac1506084c7`.

The first passing build emitted an **unused import warning** in the new test; no test failure. It was removed in source/test revision `db7257021de5a2d00dcdfb9f023d90dde5036166` without changing executable assertions. A terminal exact-source receipt on this final revision is separately required before describing it as tested. [pcc:unverified]

This is a new *independent correctness oracle*, not a faster filter: dense raw-rank computations cost O(R(C+B) min(R,C+B)) and may be prohibitively expensive in production. The result does not measure wall time, memory, genuine independent elliptic-curve relation yield or a complete cold solver. The raw rank check is specifically a small-fixture falsification gate and should not be inserted in the hot collector.

## Next executable step

On the frozen natural, group-verified Prime31 two-LP source (if/when identified), run forest/pivot/MST with identical source IDs, field ell, target column convention, caps and seed, replay each source curve identity, compare useful rank and RHS augmented rank, then charge output witness/FB payload nonzeros, processing wall time, peak RSS, refusal/fallback and full recovery. Do **not** substitute these 128 constructed finite-field systems for authentic Prime31 or extrapolate the pass into an asymptotic claim. Root-crate and signed PCC gates remain pending. No spending/merge was initiated.
