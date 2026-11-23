# Frozen protocol: Prime31 W8 shared-four-slot block scaling (2026-11-19)

**Pre-data, independently falsifiable.** This protocol must be committed before any full study or post hoc change to target seed, arm definitions or sample sizes.

## Fixed source and mathematical construction

- Pin existing `prime31-algebraic-ic` Rust 1.100.0 arithmetic to Git commit `1df714645aa683cf975bc76fcacb9842cb4780fd` (source-specific checkout).
- Unchanged prime-field ordinary cofactor-one `E: y²=x³+847514846x+2087444724 mod p`, `p=2147483647`, prime order `n=2147446831`, generator `G=(625037715,2100627251)`.
- First 31 increasing rational x values, lift to `y=min(y,p-y)`. Four fixed disjoint/overlapping windows: 0..7, 8..15, 16..23, 23..30. All four slots choose any of 8 factor-base points with independent ± signs.
- Construct the *same* signed four-slot multiset `S` with weights `[1,2,4,8]` once per window (16^4=65536 signed words).
- Exact relation family: `R = U + [2^k]V`, `U,V∈S`, `k∈{4,5,6,7,8}`; corresponding eight-slot weights `[1,2,4,8,2^k,2^(k+1),2^(k+2),2^(k+3)]`. All schedules are superincreasing, so they have no *formal integer coefficient* collisions, but some may have accidental EC collisions.
- Because the odd-order curve has `gcd(2,n)=1`, `[2^k]` is an automorphism: right-half EC fibre sizes and collision counts must equal those of `S` **for every k**. This is an exact invariant/control, not a measured outcome.

## Sampling and observed variables

- Independent, unused held-out deterministic SplitMix64 seed `0x424c4f434b534341` (`BLOCKSCA`), rejection-uniform nonzero `[d]G` target points. Derive 512 public Q sequentially, **128 Q per window**. Expose only public points to grading; the generating scalars must not enter the grader.
- For every window and k, enumerate full `m_k(Q)=Σ_{U∈S}l(U)l([2^-k](Q-U))` using a full 4+4 generic group-sum table, with **no positive early exit**, and record all 128 counts.
- Frozen primary comparison: hit rate (any witness) of k=5 vs k=4, paired on 512 Q; report all k in order but avoid post hoc cherry picking. Secondary: sum/count variance, per-window support/collision counts, build/probe original arithmetic counters, cold wall clock, selected lexicographic signed witness for independent Python affine replay, x-only doubling-chain depth k.
- Across k, allow reuse of `S` and progressively double its scaled right-hand copy. Charge all group operations; amortized cross-arm prep must be explicitly tagged and **not** presented as independent cold per-arm cost. Every returned witness is checked by original EC addition.
- Minimum exact gates: all 40 original crate library tests; 3+ focused tests; curve subgroup certificate; 512 × 5 multiplicity rows; `left_unique=right_unique` for each k; independent Python checks of target-generation integrity, 20 witness equations (one per arm/window), all summary counts and paired hit counts.
- Source-specific public GitHub Actions on Ubuntu 24.04; `cargo +1.100.0 --release --locked`. Freeze code and protocol in the PR branch; source, seed and hashes in raw artifact; no paid or cloud workload.

## Boundaries and decision rule

This is a **generic-table grading control**, NOT a public-only nonenumerative RR9/F4 extractor, NOT an admissible faster index-calculus attack, and does NOT measure whole ECDLP solving time. Higher k adds polynomial graph constraints: an x-only duplication chain requires k doubling links; clearing denominators must exclude twist/nonliftable roots and verify all signs. If k=5..8 improves yield only marginally, prioritize k=4 for the actual extractor (#4692), because a larger k cannot reduce the *minimum stated chain length*. If an arm improves substantially, propose a **separate** preregistered F4 comparison using the same public-only B8 46-positive/18-negative controls, before rank31/1024 integration and same-runner BSGS/rho.

Previously checked: #4807 (full W8 fibre), #4805 (formal signatures), #4803 (unweighted support), #4770 (ternary complete generic control), #4793/#4532 (full-cost comparator), #4692 (F4 owner). Literature: Petit–Kosters–Messeng PKC 2016; Amadori–Pintore–Sala Finite Fields Appl 2018. Research-KB MCP not available in this chat/connector runtime; Github code/docs and published primary summaries were fallback, no originality-by-absence claim.

**Only this protocol's exact counting design and planned sample are established before measuring. Do not enter results in this file.**
