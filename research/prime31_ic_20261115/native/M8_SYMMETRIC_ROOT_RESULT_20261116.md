# Prime31 m8 symmetric rational-root and infinity correctness gate

**2026-11-16, Rust 1.100.0 release on GitHub Actions, producer-development only [pcc:unverified].** No new non-generic full ECDLP solve or stronger complexity claim. Keep the verified complete APS-derived Rust 384/384/16384 single-relation control in PR #4391 intact, including its fresh 24-Q A/A.

## Result 1: actual executed field-only versus independent signed full-point equivalence

**Executed source** `d1a92b3251d06b465c38d8b486f7aaab9f827cc4`, [Actions 43828425745](https://github.com/isogeny-labs/crypto/actions/runs/43828425745), job `133167637017`, passed example unit test and standalone release program. Full, unedited source-specific JSON program output is frozen in [M8_SYMMETRIC_ROOT_EXACT_CI_20261116.json](M8_SYMMETRIC_ROOT_EXACT_CI_20261116.json). This source emitted one harmless unused-helper warning, removed in successor `1a3882044`. The execution result is pinned to the original head, not automatically transferred to successors.

| Exact original Rust cell | Candidate f | Public-target decisions | Positive decisions | Identity-positive f | S3 field calls | Diagonal infinity branches | False +/- |
|---|---:|---:|---:|---:|---:|---:|---:|
| B4 exhaustive | 165 | 660 | 200 | 35 | 5,975 | 295 | **0 / 0** |
| B8 seeded distinct | 128 | 512 | 131 | 3 | 8,925 | 81 | **0 / 0** |
| **Total** | **293** | **1,172** | **331** | **38** | **14,900** | **376** | **0 / 0** |

Three malicious-input checks per B (non-base root, nonmonic f, wrong-degree f) were rejected. Four target kinds for every f were: constructed all-positive sum, fixed [17]G, fixed P0, identity. Independent group arithmetic enumerated all signed point possibilities **after** the field-only candidate search; target scalar/point-sum data never entered the algebraic S3 recursion.

Field-only certificate plus eight-leaf S3 evaluation: B4 2,924,984 ns, B8 4,539,249 ns. Independent post-result group-identity checking: B4 8,502,694 ns, B8 12,802,130 ns. Full in-process program including source setup and both sides 29,564,569 ns. These are *small-B given-candidate validation timings*, never a target-conditioned polynomial oracle, APS/PKM cost comparison or cold ECDLP timings.

## Why the finite-field equivalence is exact (and what it is not)

Fix the same ordinary cofactor-one curve E/F_p, `p=2^31-1`, `n=2147446831` prime, first B admitted depth-six abscissae `S`. Let `H(T)=prod_(x in S)(T-x)` and any monic degree-eight polynomial `f(T)`.

`f | H^8` **if and only if** every algebraic root of f is one of the admitted x values and its multiplicity is at most eight. Because f is degree eight, this exactly represents unordered eight-element multisets with repetition from S, with no base-root order or y signs. This is an equality of *Fp-rational solution sets for f*, not an assertion of equality of ideals over all coefficient representations.

For a GIVEN f, the Rust oracle uses monic factor division to recover each x. Iterating the standard Semaev S3 quadratic against a current possible partial abscissa propagates both signed point-sum x candidates; if the two x values coincide, the linear S3 root is accompanied by the point at infinity (a cancellation branch). The identity state joined with a new abscissa returns that abscissa. By induction, the final field states equal the x/identity projections of all 2^8 signed lifts. Because global sign reversal sends any reachable target Q to -Q, abscissa-only membership implies there exists a signing giving the original full affine Q; the test separately verifies this against exact point additions. 1,172 checked decisions and 376 diagonal events exercised the implementation; this is not a general machine-checked formal proof.

The algorithm **does enumerate candidate f in the test harness**. It has no mechanism to find f from an arbitrary public target without looking through the candidate universe. It is a correctness gate for an eventual nonenumerative symmetric S9 solver, not that solver.

## Mathematical scale and stop condition

The exact unordered eight-multiset counts are C(11,8)=165 for B4, C(15,8)=6435 for B8 and **C(38,8)=48,903,492** for original B31, before any y sign enumeration. The existing completed APS 384/384/16384 Rust confirmation consumed 877,824 S3 pair trials for 24 heldout logs = **36,576 pair trials per solved Q** on that frozen panel. Exhaustive B31 candidate-f checking would require 48,903,492 candidate inspections per Q, **1,337.04× as many inspections** as those APS pair calls; they are different-cost operations and not a wall-time ratio. A claimed non-generic improvement must therefore eliminate candidates *symbolically* from the target, not enumerate their symmetric encodings and call them index calculus.

A separate **predeclared exhaustive B8 continuation** [protocol](M8_SYMMETRIC_B8_EXHAUSTIVE_PROTOCOL_20261116.md) keeps the original B8 sample intact and tests all 6,435 f / 25,740 target decisions in an opt-in Rust mode. Its source-specific execution/receipt is separately tracked. It remains a correctness test, not a security result.

## Prior work, lead ranking and next falsifiable experiments

Reviewed primary existing source rather than repeating searched areas: #4188 already has full B4 m4 pair-support quotient/norm/GCD root extraction (so no reinvention of two-pair resultant); #4373 chained S3/m8 F4 hit memory caps even for known positives; #4391 anchored two-point m6 has an exact 3.7407% support ceiling and all registered natural F4 queries censored; #4383 equal T1024 exploratory speedup failed fresh confirmation. The current #4391 APS asymmetric 384/384/16384 Rust is a **verified-complete internal engineering reference**, not a non-generic frontier break. Merged #4228/open #4349 selected-root compact quotient uses real Rust certificates on smaller-arity inputs but has no completed original B31 S9/root gate. Existing crypto-autoresearcher #3305 and cryptanalysis symmetrized-Semaev work warn against duplicate claims. Published PKM 2016 and APS 2018 already establish prime-field Semaev/index-calculus ideas; symmetry is also prior art. KB MCP and repo wrapper were unavailable in this connector-only environment; the bounded GitHub and publisher review is not exhaustive novelty clearance.

**Highest-priority next actual algorithmic experiment:** on B4/B8 implement **target-conditioned** elimination directly in eight elementary symmetric coefficients e1..e8 or 3+3+2 block-Vieta coefficients, using the constraints `f|H^8` (or block `f_j|H^{deg f_j}`), an original-ideal-certified balanced Semaev/S9 circuit, and selected-root/subresultant machinery from #4228/#4349. Compare extracted finite rational f sets to this independently verified *ground-truth evaluator*, including repeated x and infinity. A target-conditioned solution must not read candidate indices, points or prior group sums during discovery. If all complete B4/B8 outputs match, lift to the 16 frozen prime31 natural masks, measure independent relation rank/one-useful-relation recovery, then twelve unknown scalars and fully cold completed PKM/APS/native-F4/external-CAS comparisons. Optimized rho is a separate security baseline, not the primary algebraic frontier comparator.

**Stop:** A symbolic system that only tests candidate f values or exceeds the registered memory/degree cap without complete roots is not a non-generic index-calculus improvement; record all censored and negative calls. No source here meets the requested cheaper non-generic complete prime31 objective yet.


---

## Registered exhaustive B8 confirmation — actually executed (2026-11-16 UTC)

The **separately preregistered** B8 extension ran to completion on Rust release CI at exact source `6567889f5d5cd753302c37e84cf8e8e795fb50dd`: [GitHub Actions 43829426432](https://github.com/isogeny-labs/crypto/actions/runs/43829426432), job `133170883210`, **PASS**. Frozen original CLI JSON and provenance: [M8_SYMMETRIC_B8_EXHAUSTIVE_CI_20261116.json](M8_SYMMETRIC_B8_EXHAUSTIVE_CI_20261116.json). The original 128-sample result was not rewritten or retroactively called exhaustive.

| Exhaustive registered B8 cell | Executed result |
|---|---:|
| Unordered 8-leaf abscissa multisets `C(15,8)` | **6,435/6,435** |
| Signed Semaev x/identity vs full EC decisions | **25,740/25,740 exact matches** |
| False positive / false negative | **0 / 0** |
| True-positive target decisions | 6,765 |
| Multisets whose signed support contains identity | **330** |
| Diagonal S3 infinity branches exercised | **6,270** |
| Field-only S3 successor calls | **355,113** |
| Invalid f certificates rejected | **3/3** |
| Given-f field-only certificate/evaluation time | 211.906204 ms |
| Independent complete signed EC semantic validation | 484.633133 ms |
| Full Rust in-process program (including setup and checks) | **706.903124 ms** |

Together with the earlier exhaustive B4 660-decision cell, this establishes **26,400/26,400** exact field-only versus independently verified signed full-point decisions across **all B4 and B8 unordered multisets**, for the four frozen target categories. This is a stronger finite-instance *correctness* gate than the 128-B8 sample; neither gives a method for selecting f from the public target.

**Interpretation and next step:** the expected Semaev quadratic and exceptional identity semantics of the proposed symmetric factor-base quotient survived exhaustive B8 signed-point checking. The next obstacle is not sign branch correctness at small B; it is computing a target-conditioned constraint on the e1..e8 coefficients and extracting a candidate f **without** traversing the 48,903,492 B31 multisets. No polynomial original-ideal certificate or new cold unknown-log solve was produced here. An exhaustive validator that runs in 0.707 s at B8 does **not** imply a scalable non-generic algorithm at B31, nor can its small-B evaluation clock be compared to APS complete solver wall.
