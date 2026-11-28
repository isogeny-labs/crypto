# Preregistered v2: Montgomery batched-complement weighted six-slot Prime31 index calculus

**2026-11-18, before execution; [pcc:unverified].** Follows exact source-run [#44294687054](https://github.com/isogeny-labs/crypto/actions/runs/44294687054), not an after-the-fact change to the v1 protocol.

## Frozen change

Replace **only the per-point inversion loop** computing `R - L_i` for the existing left three-sum parity table by Montgomery's batch inverse over nonzero addition denominators. Preserve exceptional cases, the exact original curve, 31 first rational-x points, weights `[1,3,5,7,11,13]`, training seed `0x4d36545241494e31`, 512 training cap, target seed `0x4d36544152474554`, 1,024 public Q, 64 shifts, right parity table, relation words, modular elimination and full verification.

For all nonzero denominators `d_i`, let `p_0=1, p_{i+1}=p_i*d_i (mod p)`; compute `u=p_m^{-1}` once, then backwards `d_i^{-1}=u*p_i, u=u*d_i`. This requires **one** field inverse per batch and `3m+O(1)` extra field products for `m` denominators, not one inversion per point. The ring characteristic and exceptional addition cases are preserved. This is classical Montgomery batching; not new elliptic-curve index calculus mathematics.

## Predeclared checks

1. Exact B2 six-weight parity **scalar vs batched** equality on all 64 ordered words' sum targets and additional negative points; replay every returned witness in original EC arithmetic.
2. Original native library 40 tests and candidate focused tests on pinned Rust 1.100.0.
3. Run *both* full v1 and v2 programs on the same GitHub runner, in the two execution orders A/B and B/A, passing identical locked inputs and checking every transcript with the independently coded integer EC arithmetic checker (no target scalar in relation discovery).
4. Rank=31 and **1,024/1,024** on both; no generic fallback. Compare exact signed relation words/equations, factor-base logarithms, target recovery, failures, attempts and counts; parity and word witnesses must be equal.
5. Report median complete-program wall times, setup/training/query EC adds, field muls/inversions, AB/BA order and memory (RSS if available). Account for caller/Q generation and full verification. The upper-level science result remains **generic-equivalent**; no claim against optimized shared BSGS/rho unless those are actually measured in same-runner comparison.

The v1 baseline and v2 code paths are frozen per source commits and remain valid even if a later separate experiment explores other optimization. Publish raw transcripts (all 1,024 targets and failures), source SHA, CI run and independent replay. Treat anything without signed independent PCC as `[pcc:unverified]`.

## Prior art and distinction

- Montgomery's trick is longstanding and already used in our Rust Prime31 batched-complement and isogeny lanes (#4618, #4671); no invention claim. Source reuse makes this an optimization to a **new six-weight parity complete-solver configuration**, not a rediscovery.
- The old eight-weight B31 generic control (#4714) uses 4+4 tables and 7/12 odd residuals. The mask-only (#4716) multiset ceiling does not apply to distinct public scalar images.
- The earlier v1 six-weight [source-run](https://github.com/isogeny-labs/crypto/actions/runs/44294687054) achieved rank31 after 106 natural attempts, 1,024/1,024 with 3,717 shifted calls, **29.082809s** whole-program wall and an independent integer arithmetic replay. This is a source-run producer claim [pcc:unverified] and is not below known shared BSGS; it is the pre-optimization baseline frozen *before* v2.
- Our next genuinely algebraic extractor remains RR9 shared coefficients (#4692); a hashed group table plus batch inversion does not improve asymptotic ECDLP complexity.

No retuning, curve changes, stopping-rule changes, or publication as a non-generic algorithm.
