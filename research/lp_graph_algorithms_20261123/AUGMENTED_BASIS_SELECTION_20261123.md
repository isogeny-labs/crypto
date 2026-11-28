# Exact affine-row basis selection after LP cancellation — 2026-11-23

Lane: [draft graph-IC PR #5043](https://github.com/isogeny-labs/crypto/pull/5043), successor to merged #4988. **Pre-registered before source edits.** [pcc:unverified]. No merge, no paid compute, no publication/leaderboard promotion.

## Bottleneck and hypothesis

The existing original-Rust `projected_rank::audit_projected_rank` already performs sparse exact elimination on filtered `CompleteRelation` rows to compute useful rank, but it only emits independent *small-coefficient* row indices and an arbitrary-length list of residual nonzero RHS rows. Its redundant rows can still carry large sparse/source payloads into subsequent exact-system solves. Hypothesis: for an **explicitly closed affine system** `S x = b` over prime-order `F_ell`, the original source rows indexed by the greedy small-column pivots **plus at most one** nonzero reduced-RHS row span the original augmented row space `[S|b]`. Keeping original rows (not reduced linear combinations) preserves provenance. This is a costed potential row-count/fill reduction, not yet a faster complete solver.

## Derivation and assumptions

Run exact sparse echelon on `S`, reducing `b` by the same pivot factors. Each original augmented row reduces to either (1) a new pivot row (record original index), (2) the zero vector, or (3) `(0,...,0; delta)` for `delta != 0`. All type-(3) residuals span the **same 1-dimensional** last-column subspace. Keeping one such original row, alongside all type-(1) original rows, gives an augmented row-space basis: `rank([S|b]) = rank(S) + 1_{some delta!=0}`. Hence the kept and original systems have identical solution sets, including the empty set if inconsistent. Never silently label a residual RHS an invalid relation when a target logarithm/unknown was hidden in the RHS; all unknowns must be explicit columns and the RHS must be known. If a contradiction exists, this remains a diagnostic/refusal for a consistent ECDLP solve.

A second pass copying only retained original `CompleteRelation` rows must count cloned sparse coefficients, original source-provenance coefficients and allocation. It must not invent a speedup from fewer rows without charging the exact-echelon selection work. No asymptotic improvement follows: the audit itself may incur large fill and ordered-map costs.

## Frozen test, control and budget

Control: existing untouched `audit_projected_rank` ranks and the full original augmented matrix. Candidate: a new additive opt-in basis selection API returning original source-bearing rows, with exact selected counts and indices. Inputs: fixed F7 duplicate/pivot/RHS contradiction fixture plus >=128 deterministic synthetic canonical complete-relation batches across F3, F7, F101 and F2147446831; original `small`, `rhs` and `sources` preserved. Check: independent dense rank on `S` and `[S|b]` for original and retained subsets; exact same rank, `retained.len() == augmented_rank`, ordered distinct original indices, replay-provenance equality, correct all-zero and inconsistent cases, default evaluator compatibility. Reject noncanonical rows and nonunits as before. The bounded source-exact hosted original-Rust graph harness is the only permitted execution resource; one CI build at a time, no paid jobs or concurrent timing comparisons.

Falsification: any change of augmented rank, omission of contradiction, source witness mutation, change to existing rank classification or source-default evaluation. Runtime, memory, authentic Prime31 relation yield and full cold solve remain separate, unmeasured gates. Keep normal-equations singularity caveat.

## Sources / coordination

Checked current `AGENTS.md`, `docs/ACTIVE.md`, graph catalog README/SOURCES, current PR #5043 and merged #4917/#4905/#4988, original Rust `large_prime_graph/projected_rank.rs`, `eval.rs` and `tests/large_prime_graph.rs`. Neighbor searches: `cryptanalysis` prime-orbit large-prime collector and `crypto-autoresearcher` large-prime/IC ideas; they provide neither a source-certified Prime31 two-LP stream nor this implementation's correctness receipt. Diem (2006), https://arxiv.org/abs/math/0606607 and Cavallar (2000), https://ir.cwi.nl/pub/4456 give double-large-prime/filtering context; this basis extraction is elementary field linear algebra and carries no novelty claim. KB and Conductor MCP names/CLI are unavailable in this runtime; this is a bounded source/literature fallback, not an absence finding. No independent worker runtime exposed; one coordinator. Concurrent PRs/worktrees not edited.
