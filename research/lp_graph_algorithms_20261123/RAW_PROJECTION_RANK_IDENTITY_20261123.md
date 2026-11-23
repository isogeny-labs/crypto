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
