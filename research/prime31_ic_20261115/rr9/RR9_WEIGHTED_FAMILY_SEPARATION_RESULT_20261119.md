# Prime31: fixed RR9 and weighted W8 are different relation languages (source audit)

**Date:** 2026-11-19. Continuation of [PR #4812](https://github.com/isogeny-labs/crypto/pull/4812) and direct handoff to [#4692](https://github.com/isogeny-labs/crypto/pull/4692) and [#4816](https://github.com/isogeny-labs/crypto/pull/4816). **Scientific scope:** mathematical nontransfer lemma and an original-curve planted-control audit. No public-only extraction or whole-solver speedup. [pcc:unverified]

## The actionable result: no negative transfer from #4816 to weighted #4728

The #4816 B8 **fixed-squarefree, unweighted** Riemann–Roch extractor fixes the eight-root polynomial `M8(X)=H_B(X)` with exactly the eight distinct factor-base x values, each appearing once, and solves `R=Σ ε_i P_i`. It thereby has at most `2^8=256` signed group sums for a fixed B8. On the original curve the 256 are distinct. Its population hit fraction for uniformly random R is **exactly** `256/2147446831 = 1.1921133334e-7`. Its expected `~7.55e7` walks for 9 independent hits is due to this *specific* sparse support. Its complete toy-curve nonenumerative ECDLP success and its negative Prime31 cost result are still valuable for **that** language.

The #4728 and #4812 **position-weighted** B8 relation is instead

`R = Σ_(i=0)^7 [2^i] S_i`, with `S_i ∈ {±P_j : j in a fixed eight-point window}`.

It has `16^8=4294967296` ordered signed words. Since `#E(Fp)=n=2147446831`, the **exact group-population average ordered witness count** is `16^8/n = 2.000034289`; this is **not** a hit probability. The four-window original-crate held-out relation grading in #4807 recorded 902/1024 hits with powers-of-two weights on its particular natural public-target panel (not an algebraic extraction result). The empirically observed coverage 0.88086 is about **7.39 million times** the exact fixed-squarefree support fraction, but this is **not** a matched-sample population ratio or an algorithm-speed ratio.

**Exact mathematical relationship.** Write `Z_i=[2^i]S_i`. Then the weighted sum is indeed an *unweighted* sum of transformed points `Z_i`. However, its polynomial for a fixed witness is `M8(X)=∏_(i=0)^7 (X-x(Z_i))`, not the fixed `H_B(X)=∏_(j=0)^7 (X-x(P_j))`. Each `Z_i` must be selected from a **different slot-specific transformed factor-base set**. Neither the fixed `M8=H_B` solver's operation count nor its `256/n` hit probability applies to that slot-dependent relation system. This is a precise problem-language distinction, not a claim that the weighted polynomial system is easy or already solved.

## Original-curve planted audit: independent Python preflight

Frozen [protocol](RR9_WEIGHTED_FAMILY_SEPARATION_PROTOCOL_20261119.md) was committed before source code (protocol commit `289626d29f27a3e3da7edc34f2473e75effd8201`). The existing [original-crate Rust example](../native/examples/prime31_rr9_weighted_family_separation.rs) reuses `Arithmetic`, `check_curve` and `PRIME31`. A **separately authored Python bigint affine-curve implementation** was used as a preflight. Curve: `p=2147483647; a=847514846; b=2087444724; n=2147446831`, generator `(625037715,2100627251)`. Seed `0x5252395f57454947`, SplitMix64; four B8 windows; 128 shuffled, signed squarefree fixtures each; exactly 512 planted positives **for the weighted relation language**.

| Window | Fixed-squarefree group support | Weighted planted targets | Weighted target in fixed support | Transformed x-root multiset equals fixed H_B roots |
|---|---:|---:|---:|---:|
| 0 (indices 0–7) | 256 | 128 | 0 | 0 |
| 1 (indices 8–15) | 256 | 128 | 0 | 0 |
| 2 (indices 16–23) | 256 | 128 | 0 | 0 |
| 3 (indices 23–30) | 256 | 128 | 0 | 0 |
| **Total** | 4 distinct window supports | **512** | **0** | **0** |

For all 512 cases the unweighted same-leaf signed sum was in the fixed 256-point squarefree support (positive control), and the generated weighted sum was on the original curve. **These are planted controls**, so 0/512 is not an estimate of uniform-target coverage and does not prove universal disjointness.

Example, window 0, case 0: the signed leaf x sequence is `[0,9,5,6,1,8,3,2]`. The true weighted target is `(56922545,233314463)` whereas the corresponding unweighted target from identical leaves is `(102991796,1708216169)`. The transformed weighted root sequence is `[0,1509391484,1105745703,995697649,65723932,1592489495,571610884,1708353735]`, not the fixed window polynomial roots `[0,1,2,3,5,6,8,9]`. These coordinates were checked with independent Python curve arithmetic; source-specific original Rust CI is the **separate** gate, not presumed passed here.

## Additional exact negative result: local F3 Legendre pruning is vacuous

The predecessor #4812 proves for its eliminated x-only Horner step `E(u,z,r)` the polynomial identity
`disc_r(E) = 64 f(u) f(z) M(z)^2`, where `f(x)=x³+ax+b` and `M` is the doubling y-numerator. When `u=x(U)` and `z=x(Z)` are **ordinary rational nonidentity** points of this original odd-order group, `f(u)=y_U²`, `f(z)=y_Z²`, and `M(z) != 0` (there are no rational 2- or 4-torsion points). Therefore `disc_r(E) = (8 y_U y_Z M(z))²` is **always a nonzero square in Fp**. Every valid lower-level rational prefix candidate survives the quadratic discriminant / Legendre-square test: when the quadratic leading coefficient is nonzero it has exactly two rational roots; when it vanishes the remaining nonzero linear polynomial has one finite rational root. So **locally filtering by this character does not prune any legitimate already-rational suffix branch**, and the previously suggested "character-pruned doubling chain" should not receive a dedicated performance experiment without an additional *target-conditioned* character obstruction. This is a direct corollary of verified symbolic identities, not a new F4 measurement. Replacing generic 4+4 grading with Legendre checks alone cannot produce the required extractor throughput.

## Quantitative whole-solver admission gate

The previous complete **generic-grader** weighted rank31/1024 target cohort #4728 used **1,312 shifted relation requests**. The best fully charged shared-million-baby BSGS reference #4532 took **170.113 ms** for the same curve and 1,024 public logs on a prior matched run. Thus **even assuming every other index-calculus stage costs zero**, a future weighted nonenumerative extractor would require

`T_extract_mean < 170.113ms/1312 = 0.1296593ms ≈ 129.66µs per attempted public residual`.

The actual bound is *strictly smaller* after charging factor-base setup, failed queries, rank defects, linear algebra, logarithm recovery, verification, and memory. This is a **necessary budget bound from historical data, not a measured extraction speed**, and a decisive head-to-head still requires **the same runner**. On the historical W8 cohort, an extractor requiring even 1 ms/residual consumes >1.312 seconds before any other costs. Therefore the ranking bottleneck is **public-target-only extraction throughput**, not another 3–10% generic grading coverage improvement.

The #4816 B8 7.35-ms quotient figure cannot be imported as a weighted extractor timing; its polynomial system has a different witness language. Likewise the #4816 unweighted 10^9-fold expected-work gap against rho does **not** close the weighted lead. It **does** warn that simply translating an unweighted RR9 Gröbner formulation without an extraction-cost breakthrough will miss the fully charged budget by orders of magnitude.

## Prior-work and negative lead audit

- [#4816](https://github.com/isogeny-labs/crypto/pull/4816), `research/prime31_rr9_and_closures_20261119/REPORT.md`: fixed-squarefree/quotient Riemann–Roch exact B4/B8 fibres and a full toy ECDLP, extremely unfavorable Prime31 costs when all natural misses are charged. **Relevant negative control, not a weighted W8 benchmark**.
- [#4803](https://github.com/isogeny-labs/crypto/pull/4803): sign-cancellation bound 187137 for unweighted repeated eight-leaf support; this is a third, distinct witness language.
- [#4728](https://github.com/isogeny-labs/crypto/pull/4728), [#4807](https://github.com/isogeny-labs/crypto/pull/4807): complete rank31 generic-grader weighted pipeline and held-out W8 grading. The latter is not a non-generic extractor.
- [#4812](https://github.com/isogeny-labs/crypto/pull/4812): weighted signed x/y Horner and x-only 14-variable eliminated systems, correctness and rational splitting only. **No F4 timing**.
- [#4692](https://github.com/isogeny-labs/crypto/pull/4692), [#4822](https://github.com/isogeny-labs/crypto/pull/4822): existing public-only RR9/F4/Wiedemann research; coordinate and do not fork their implementations. The fixed-8-root system must not be substituted for the slot-weighted one.
- [#4532](https://github.com/isogeny-labs/crypto/pull/4532): optimized shared BSGS 170.113-ms reference, genuinely benchmarked against the previously complete mixed IC solver.
- Published comparators: Petit–Kosters–Messeng, *PKC 2016*, DOI [10.1007/978-3-662-49387-8_1](https://doi.org/10.1007/978-3-662-49387-8_1); Amadori–Pintore–Sala, *FFA 2018*, DOI [10.1016/j.ffa.2018.01.009](https://doi.org/10.1016/j.ffa.2018.01.009); Kudo–Yokota–Takahashi–Yasuda, *Acceleration of index calculus for solving ECDLP over prime fields and its limitation* (2019). APS already reduces factor-base relation burden and the hybrid Gröbner literature shows that hybridizing is established prior art, not automatic novelty.
- Shared research-KB service was not exposed in this runtime; GitHub primary documents and selected published source metadata were used as a bounded fallback. No statement that this separation is new to the literature.

## Priority experiments and pass/fail gates

1. **P0 weighted public-only grammar equivalence (#4692):** Construct the actual eight slot-specific transformed-root constraints or exact signed Horner chart; forbid hidden signs, index masks, original-curve sum tables and planted labels inside the candidate extractor. On frozen 46-positive/18-negative natural B8 residuals, recover **all** signed indices for positives; correctly classify negatives; separately handle timeouts as unknown, not false. Verify `Σ [2^i]S_i = R` with independent original-group arithmetic. This must not reuse the fixed `M8=H_B` gate from #4816.
2. **P1 cost gate:** Measure absolute attempted-residual wall time and field ops on same host for 14-var degree-10, 21-var degree-4, seven-coefficient weighted RR9 and sparse remainder-tower variants. Pre-register a hard speed requirement of **<129.66µs mean** before setup and reconstruction on the historical 1,312-request count; failing this does not invalidate a correctness component, but blocks a 170.113-ms 1,024Q win under that count.
3. **P2 complete solver:** Replace only the generic grading oracle in existing rank31/1024 original Rust #4728, reuse relations/rank/matrix/recovery and measure setup, failures, memory, Rust wall, fresh target verification, compared to same-runner strongest IC, shared BSGS and optimized rho for 1,64,1024 targets. Must independently sign PCC if a competitive claim is made.
4. **P3 independent scaling:** Repeat on ordinary cofactor-one prime-order curves of increasing size without deliberately weak parameters. Only a measured total advantage on Prime31 warrants a full asymptotic cost discussion.

**Status:** Audited a materially important unsupported transfer between two prior programs; independently executed original-curve Python planted controls; implemented original Rust audit and CI gate. A cheaper **complete** prime-field index-calculus solver is not shown. Exact-source CI status must be checked separately before reporting Rust results as verified. [pcc:unverified]
