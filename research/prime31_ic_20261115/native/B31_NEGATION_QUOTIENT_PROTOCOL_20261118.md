# Prime31 negation-quotient paired-complement experiment — frozen protocol (2026-11-18 UTC)

## Question and mathematical invariant

For the unchanged ordinary, cofactor-one curve E/F_2147483647: y^2=x^3+847514846*x+2087444724 and n=2147446831, G=(625037715,2100627251), can the **existing generic-equivalent Rust signed 4A+4A index-calculus pipeline** be accelerated by quotienting the *left* signed four-sum index by the involution P↦−P?

The signed 4A set S satisfies S=−S. For every nonidentity P in S, store one canonical representative P with y(P)≤p−y(P), plus both original 4-leaf words for P and −P. Store identity once. For each public residual R and canonical P test BOTH R−P and R+P against the ORIGINAL 4A hash map (preserving exact 8-leaf witnesses). The two additions share the same nonzero x-denominator (x_R−x_P). Batch its inverse across 512 canonical points; compute both slopes and group outputs. For x_R=x_P or infinity, use original Rust curve arithmetic instead. The identity is fixed by the involution, and this odd-order group has no nonzero point of order two. Completeness is exact: if R=L+H with L,H∈S, then either L=P or L=−P for a canonical P∈S, so one of the two lookups succeeds.

This is an optimization of a **generic-equivalent** index-calculus control, not sign-independent RR9/S9, not a non-generic attack, and not a claimed new index-calculus asymptotic. Montgomery batching and negation symmetry are established ideas; the measured improvement, if any, is specific to this implementation.

## Frozen test plan and metrics

1. Reuse source at `research/prime31_ic_20261115/native/examples/prime31_s9_b31_half_sum_oracle.rs` and `prime31_algebraic_ic` native Rust arithmetic, not a new group implementation. Original Rust 1.100.0 native crate is pinned at 1df714645aa683cf975bc76fcacb9842cb4780fd in the prior workflow. **Do not** change the curve or generator.
2. Build a signed four-sum index from the *first 31 rational x* factor base (historical winning 256Q control, #4635). Require 618885 distinct four-sum points and exactly (618885+1)/2=309443 canonical representatives including infinity, with proof/validation that negation maps inside S.
3. Baseline original 512-lane batch complement versus paired candidate on the same **256 public first-shift residuals**, from frozen seed 0x4d49583450333131; require baseline 244/256 first-shift successes and exactly equal hit/miss masks, exact eight-point group replay of every new witness, exhaustive negative checks, and exact special-case checks (R=O, R=P, R=−P, L=O). No secret scalar reaches either membership oracle. Add planted positives only as checks, not speed-ranking trials.
4. Compare charged preparation time, query wall time, custom field multiplication count, inversion windows and visited candidate half-sums. Wall is a same-runner diagnostic; reverse AB/BA order for another run before promotion. Include failed lookups in both costs; do not suppress rare exception branches.
5. If component is correct, evaluate a **complete blinded 256-log factor-base/index-calculus solve**, with rank=31, 16 extra known-source relation controls, all public shifts/failures, all final [k]G=Q verifications, and any generic fallback fully charged. Compare same-runner existing Rust first-x control, do not import historical timings as a matched ratio.
6. Adversarial/structural falsifiers: any different hit mask; any failed signed witness replay; any unprocessed infinity/same-x case; nonexhaustive negative; rank<31; mismatched logarithms; higher charged cost. Record outputs, raw JSON, CI provenance. Do not retune the corpus after seeing results.

## Prior-lead and literature audit

- Reuses #4618 batch inversion, #4623 complete 1024-Q Rust, #4626 matched B31 bases, #4629 base size and #4635 first-x baseline; no duplicate general RR9 effort.
- #4604 owns sparse RR9 D9 negative certificate; #4575 B8 generic 4+4; #4581 seven-quadratic split; #4596 matched 1024-Q IC-vs-BSGS; do not duplicate these experiments.
- Published prime-field index-calculus: Petit/Kosters/Messeng (PKC 2016, https://doi.org/10.1007/978-3-662-49387-8_1), Amadori/Pintore/Sala (FFA 2018, https://doi.org/10.1016/j.ffa.2018.01.009). These do not make a generic group lookup a non-generic cryptanalytic method.
- Knowledge-base MCP and its repository CLI are not available via this connector-only runtime. This is a bounded review, not a novelty clearance.

Frozen before candidate source edits. No paid compute, external secrets, merge or leaderboard promotion. Verification is producer-side until separate signed PCC validation.
