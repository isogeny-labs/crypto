# Matched full-cold prime31 Rust APS S3 versus batched-Jacobian BSGS — executed negative result

Date: 2026-11-16. **Source-specific developer evidence, [pcc:unverified].** Experiment implemented and tested in the existing Rust `prime31_aps_one_relation.rs`, not a new language reimplementation. The benchmark does **not** establish a cheaper non-generic ECDLP solver; it reveals an important baseline gap under matched whole-solver costs.

## Predeclared test, source and execution

- [Frozen protocol](APS_VS_BSGS_COLD_PROTOCOL_20261116.md) was committed **before code** as `147e0665bdab5af45203bf04e63a6bb4a6074c7e`, with 24 distinct caller-known-only public Q generated deterministically from 0x42475353434f4c44.
- Source implementation commit `f7f6e3896f7abce5c9761267870782a173849554`; locked Rust 1.100.0 release tested at exact workflow-head **`3bdb844a52987ac6f798788b8a77544e507dcc33`**.
- [GitHub Actions run 43888222148](https://github.com/isogeny-labs/crypto/actions/runs/43888222148) job `133372011820` returned **SUCCESS**. Focused existing S4/F4/B8/APS tests and the new `--aps-vs-bsgs-cold` and known-answer Rust unit test all passed.
- [Frozen unmodified program JSON](APS_VS_BSGS_COLD_CI_20261116.json) contains all 24 × 3 individual complete cold runs, scalar equations, public targets, per-round APS cost, BSGS table size and giant counts. Saved from the job output with only GitHub timestamp prefixes removed, in evidence commit `0f41e58cc953e2cecafc5e07ee1c606bb64ad283`. Independently parsed within producer session: 24 cases, 72 correct caller-known-answer k, all `[k]G=Q` checks, all sums and win counts match the program summary. **This is not an independent PCC or cross-machine reproduction.**

Exact same ordinary cofactor-one curve for both methods:

```text
p = 2147483647 = 2^31-1
E: y^2 = x^3 + 847514846*x + 2087444724 (mod p)
#E(Fp) = n = 2147446831, prime; cofactor 1
G = (625037715, 2100627251)
APS partitions = [384,384,16384]; max six rounds
BSGS m = ceil(sqrt(n)) = 46341; 46340 nonidentity baby points per cold Q
```

## 24 new targets, fully cold per-target Rust execution

| Complete solver arm | Correct logs | Sum of 24 in-process cold runtimes |
| --- | ---: | ---: |
| Existing APS 2018-labelled one-relation S3 factorized/x-fibre/Jacobian | **24/24** | **156.908627 ms** |
| **Independent complete Shanks BSGS, fresh baby table for every Q** | **24/24** | **122.564943 ms** |
| BSGS, independent A/A repeat, fresh baby table every Q | **24/24** | **121.326218 ms** |

**BSGS is 21.8877% cheaper (1.280208× as fast)** as the current APS implementation on this matched prime31 panel; 18/24 per-target BSGS paired wins, APS 6/24. BSGS A/A aggregate drift **1.0107%**. Each arm independently reconstructed its complete unknown scalar; the caller-held k was used only afterward to verify results. No partial algebraic oracle, shared factor-base preprocessing, intentionally weak curve, uncharged failed attempt or selected successful target.

APS measured 1,022,592 S3 pair queries in 24 one-round completes; paid public-source setup 66.652306 ms and pair-root plus candidate verification 81.353766 ms (remaining time in end-to-end bookkeeping and other charged operations). BSGS charged 88.291244 ms total baby construction and 34.114140 ms giant-walk plus replay across 24 targets. Its baby table is a genuine O(sqrt(n)) memory burden, **not** evidence of a better generic complexity exponent. The measurements are *in-process cold* and do not charge OS process or compilation startup, equally for both methods.

## Classification and prior-lead check

This **corrects a gap** left by successive relative APS-vs-old-APS engineering wins in the existing #4391. It does not invalidate those paired implementation improvements; it shows that this source has **not** beaten the matched BSGS control, much less the best native negation-map rho or full external PKM/APS solver.

- The earlier `research/prime31_ic_20261115/BATCHED_INDEX_CALCULUS_FOLLOWUP.md` stated old IC remained about two orders of magnitude behind then-optimized BSGS; that comparison did not use this source. This protocol reused the Rust arithmetic to test the changed head rather than extrapolating the old denominator.
- [PR #4383](https://github.com/isogeny-labs/crypto/pull/4383) already rejected an apparent equal-T tuning win on fresh targets; the two post-Jacobian profile sweeps in #4391 preserved [384,384,16384] after both lost. Do not repeat parameter cycling without a changed mechanism.
- [PR #4373](https://github.com/isogeny-labs/crypto/pull/4373) chained S3/S4 eight-leaf F4 remained **censored**, not completed, while fixed two-anchor six-leaf support is bounded above by 3.7407% on the B31 base.
- [PR #4228](https://github.com/isogeny-labs/crypto/pull/4228) and [PR #4349](https://github.com/isogeny-labs/crypto/pull/4349) supply exact-quotient/selected-root tools, but a target-conditioned B31 original-ideal-certified symmetric S9 root extractor has **not** been established.
- The existing `M8_SYMMETRIC_ROOT_RESULT_20261116.md` exhaustively validated 26,400 B4/B8 *given-f* target decisions; it did not extract `f` from unknown Q. At B31, exhaustive eight-multiset `f` lookup has C(38,8) = **48,903,492** candidates, not a scalable nonenumerative oracle.
- Cross-repo `isogeny-labs/crypto-autoresearcher/experiments/semaev-alternatives-20261105/SUMMARY.md` and its errata already reject unchanged Vieta-conic sieve (SA-02), m4 solver family (SA-15b), and tower tuning (SA-14b), with other limited open structural leads. Avoid duplicate relaunch.
- Literature: Amadori–Pintore–Sala, FFA 2018, DOI [10.1016/j.ffa.2018.01.009](https://doi.org/10.1016/j.ffa.2018.01.009); Petit–Kosters–Messeng, PKC 2016, DOI [10.1007/978-3-662-49387-8_1](https://doi.org/10.1007/978-3-662-49387-8_1). One relation via labelled public sources is APS prior art; this explicit finite-list S3 enumeration is **generic meet-in-the-middle equivalent**, not a newly proven non-generic attack. KB MCP not accessible in this runtime; search used repo sources, PRs, published papers and cross-repo atlas, with no exhaustive novelty clearance.

## New experiments, in value order

1. **True target-conditioned symmetric eight-leaf selected-root solver**: reuse #4228/#4349 and the already exact B4/B8 S3/EC verifier, construct 3+3+2 block-Vieta rational factor-base constraints in coefficient variables, produce original-ideal-certified nonenumerated roots for natural positive/negative Q including cancellation and infinity, and match *complete* finite rational root sets against the B4/B8 checker. Source indices, group sum tables and planted witnesses must not leak into discovery; censored F4 is undetermined. Failure to return roots invalidates the method.
2. **Scale from B4/B8 to B31**: reject any implementation whose root selection enumerates the 48,903,492 candidate multisets or hides an equivalent materialized quotient, then count useful signed independent relations/rank, complete unknown scalar solves, failed queries and all cold costs on 24 novel natural Q.
3. **Fresh separated APS engineering gate**: change the third-list index from `HashMap<u64,Vec<usize>>` to collision-free-per-source `HashMap<u64,usize>` or measured open addressing, exploiting the existing global x-unique invariant; measure a preregistered 24-target factorized-S3 full-cold comparison against old source and cold BSGS. This is *generic-equivalent optimization only*, not the main scientific breakthrough; reject if ≤1x or within A/A noise.
4. **External complete controls and scientific gate**: matched same-input PKM/APS/F4/Singular/Magma completed-oracle tests, optimized negation rho, memory/RSS peak, and a genuinely independent PCC replay of every archived scalar/timing claim before leaderboard admission or security conclusions.

**Decision:** preserve the successful existing Rust APS solver as an IC-derived implementation control, record the negative BSGS comparison, prioritize the target-conditioned nonenumerative S9 mechanism, and do not claim the user's cheaper non-generic-complete ECDLP goal has been met.
