# Exact useful projected rank after LP cancellation — preregistered native gate

Date: 2026-11-23 UTC. Lane: draft PR #4988. Status: proposed / unexecuted as of this preregistration. [pcc:unverified]

## Mathematical target
Let L in F_ell^(R x C) be the large-prime incidence/coefficient matrix, S in F_ell^(R x B) the original factor-base coefficient matrix and b in F_ell^R its scalar RHS. The already implemented forest emits an explicit basis W of ker(L^T), retaining source coefficients and replayable original-row IDs. The resulting useful factor-base system is T = W S, with RHS u = W b. It is **not** valid to count all d = R - rank(L) emitted LP dependencies as useful: useful rank is rank(T), which can be zero even when d > 0. A nonzero left-kernel row that gives (0,u_i) with u_i != 0 identifies an inconsistent projected system, not an independent useful relation. Authentic group validity remains a separate prerequisite.

This iteration adds a sparse streaming exact-field rank audit to the existing native Rust graph lane. It selects the original indices of rows whose *small-coordinate* coefficients increase rank, without losing their witness/source records. It reduces each canonical sparse row against stored normalized leading-pivot rows, carrying RHS through the same elimination. A reduced zero-coefficient row with nonzero RHS flags a contradiction; it must not be counted as new small-coordinate rank or silently promoted to a valid equation. A contradiction indicates incompatible input or arithmetic assumptions; it is not itself a group validation oracle. For one RHS column, rank([T|u]) = rank(T) + (any contradiction ? 1 : 0). All arithmetic uses F_ell, not coordinate F_p. The modulus must be prime for rank theorems; composite odd modulus is allowed by the parent API but noninvertible pivots must fail closed.

## Fixed inputs and acceptance
- Control: existing LpIndex + collect_witnesses_lazy + materialize_complete_relations. Candidate: new audit_projected_rank over *the exact replayed CompleteRelation output*, preserving its source row IDs and original small rows. No filtering or solver behavior changes in this step.
- A four-row synthetic F7 LP stream with identical LP coefficient and small scalings 1,1,2,3, consistent RHS 2,2,4,6: LP left-nullity 3, useful small rank exactly 1. Retain all three emitted witnesses; only one small row is independent. A duplicated small coefficient with changed RHS must flag a contradiction.
- Deterministic multi-field canonical sparse synthetic rows: compare small rank and augmented rank with the existing independent dense_rank oracle, compare the rank of selected original rows with the full rank, include zero rows, duplicates and free columns. Refuse noncanonical coefficients, duplicate/unsorted support, RHS out of range and composite nonunit pivots.
- Source-exact native Rust harness, existing GitHub-hosted workflow, 12 minute cap. No isolated wall-clock benchmark, paid compute, merge, or leaderboard change authorized. Full unshimmed root-crate build and authentic p=2147483647, ell=2147446831 group-valid inputs remain separate gates. No novel asymptotic claim.

## Cost model and falsification
Charge sparse input traversal, exact BTreeMap row elimination and normalization, coefficient multiply-add terms, modular inversions, persistent pivot nonzeros and original witness materialization separately. Worst-case fill still can be quadratic in factor-base width; an explicit small-rank audit is **not** a faster complete ECDLP solver. If any selected original subset loses dense rank, an inconsistent RHS is unreported, or a malformed row is accepted, the candidate fails. No benchmark claim follows from passing synthetic tests.

## Sources checked
- Repository AGENTS.md, docs/ACTIVE.md, docs/session-logs/2026-11-23.md, docs/ic/graph-algorithms/{README,SOURCES}.md and current PR #4988 original Rust witness/replay implementation.
- isogeny-labs/cryptanalysis: experiments/ic-assumption-graph/REPORT-2026-10-06.md (rank accumulation context) and experiments/ic-bench/prime_bridge.py (coefficient-rank versus structural assumptions).
- isogeny-labs/crypto-autoresearcher: bounded code-search for LP/rank; no direct owning implementation identified by those hits, not an absence proof.
- Cavallar, *Strategies in filtering in the number field sieve* (2000), https://ir.cwi.nl/pub/4456; Diem, *Index calculus with double large prime variation* (2006), https://arxiv.org/abs/math/0606607. These motivate filtering and rank checking but do not prove prime-field ECDLP gains.
- Sources checked (KB): KB unavailable — research-KB MCP server and repository wrapper/checkout are not exposed in this runtime. No source_id receipt claimed. Conductor command and independent worker spawning similarly unavailable. Current ownership is the already open draft graph-IC lane; no new worktree or competing build was launched.


## Source implementation submitted for native qualification

Committed in draft PR #4988: `src/cryptanalysis/large_prime_graph/projected_rank.rs` adds a sparse echelon diagnostic, exposed via `large_prime_graph::projected_rank::audit_projected_rank`. It retains every original `CompleteRelation`, returns selected original indices and explicitly segregates redundant rows from inconsistent RHS rows. It tracks pivot nonzeros, peak retained-plus-transient nonzeros, field elimination and normalization operations and inversions. The field-order modulus is a caller obligation; each required nonunit pivot is refused. The code path does not change the production collector, LP witness construction, filter or target solver.

Four test additions in `tests/large_prime_graph.rs`: exact F7 LP-nullity-vs-useful-rank and original-source replay; planted RHS inconsistency; 128 fixed-seed randomized sparse systems over F3/F7/F1000003/F(2^61-1) compared to independent dense small/augmented rank and rank of selected original rows; malformed input and composite nonunit refusal. Synthetic fixtures are not authentic collected Prime31 relations or cold timing. The PR's source-bound CI automatically runs on changed graph/test files. Do not report native qualification until it completes.

This is a direct response to the missing *useful independent factor-base rank* observable in previous synthetic LP witness tests. Follow-up, after source-exact native pass: matched pivot/MST/forest on an eligible, frozen, group-replayed Prime31 source stream, including exact rank(T), RHS consistency and all materialization costs. A rank audit alone is not a solver acceleration. [pcc:unverified]

## Mathematical interface qualification (important boundary)

The rank/RHS consistency interpretation assumes the right-hand side is a known scalar and **every unknown logarithm, including the target logarithm**, appears as an explicit matrix column. If a collector instead encodes `alpha + beta*d` in the scalar RHS while keeping unknown `d` hidden, a nonzero reduced RHS is **not** evidence of invalid group relations. Re-encode `beta` as a protected target column and retain the known `alpha` as RHS, or treat this diagnostic as informational only. The implementation documents the precondition and a separate sparse-column test uses column ID `u32::MAX` to rule out any implicit dense allocation. This audit does not substitute for the target-uniqueness work in draft PR #4972 or final `[d]G=Q` replay.

The initial source/test commit `6546a06b` ran in source-bound GitHub Actions [#45197339235](https://github.com/isogeny-labs/crypto/actions/runs/45197339235): **33/33 release Rust tests passed**, including all four new projected-rank tests, job `137771219897`, 2026-11-23 14:45 UTC. The workflow originally hashed only the parent graph/witness and existing filter/test sources. Follow-up commit `9736b33` extended the workflow to hash the new `projected_rank.rs` and changed the source trigger to cover the whole graph child directory; `28a94ac` clarified the hidden-target limitation; `656f64b` added the fifth regression. These source/test/workflow changes require a new terminal native receipt before attributing 34 tests to the final revision. No external PCC signer or complete solver run was invoked. [pcc:unverified]

## Final exact-source native gate — PASS (2026-11-23 14:50 UTC)

[GitHub Actions run #45198139106](https://github.com/isogeny-labs/crypto/actions/runs/45198139106), job `137774010764`: **34/34 release Rust integration tests PASSED**, zero failed, with original production graph, projected-rank, witness, pivot/MST filter and original test sources linked by the native harness. The five new projected-rank tests all PASSED: source-replayed F7 LP-vs-small rank; RHS inconsistency; 128 deterministic dense-rank/augmented-rank/selected-original-subset checks; noncanonical and F9 nonunit refusal; explicit large target-column counterexample. Rust compiler/toolchain installed by `dtolnay/rust-toolchain@stable`. Workflow syntax CI passed separately. [pcc:unverified]

The CI original-source SHA-256 inventory was:
```text
48070937d7c7a6d046c545438c1a40794bec5d544c63f075f6bd48b35ec8d597  src/cryptanalysis/large_prime_graph.rs
80dfe96e62779b661ca5a28a593ac91c65b23ec022d90f9a996e48d92b304fdc  src/cryptanalysis/large_prime_graph/witness.rs
9c7c2cef51323ce9be930fdf4c8142002d13c55902b9c642eabf47c3747a1fec  src/cryptanalysis/large_prime_graph/projected_rank.rs
b963fb4796cd9b314c5564f2f95672acdfc3c6528e977e590f681a25cdea5064  src/cryptanalysis/large_prime_filter.rs
20f662bf5b720f8f68816098a2971e00fe7a0176ea846dc345162d40bc565165  src/cryptanalysis/large_prime_filter_mst.rs
ae63445b26253a517de4b745eb793b37f929e92d3114254d648d894c22581288  tests/large_prime_graph.rs
```

The final input source/test revision was `656f64b6cedc4f0a65f4c428dcf95d92752a84d8`, with no later production-source/test changes; subsequent commits only update protocol, workflow metadata or shared handoff. This is **source-bound module qualification**, not unshimmed full-root `cargo test --release --lib`, external signed PCC validation, or naturally collected Prime31 execution. The stream-level control demonstrates LP nullity 3 but useful small rank 1 in synthetic F7. It is an algebraic correctness check, not elapsed-time or complete cold solver measurement. No paid compute or merge. [pcc:unverified]
