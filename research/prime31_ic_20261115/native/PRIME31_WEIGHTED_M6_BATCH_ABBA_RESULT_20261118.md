# Weighted M6 on original Prime31: 7.37× *paired whole-solver* Montgomery batch-inversion improvement

**2026-11-18. Reproducible native Rust and separate BigInt arithmetic replay PASS; [pcc:unverified] pending signed independent PCC. This is a complete, generic-equivalent index-calculus-style solver, not a non-generic ECDLP speedup.**

## Scientific result, exact scope

The pinned original ordinary, prime-order, cofactor-one curve is

```
p = 2147483647 = 2^31-1
E: y^2 = x^3 + 847514846*x + 2087444724 (mod p)
n = 2147446831 (prime), h=1
G = (625037715, 2100627251)
```

The implementation reuses existing Rust `Arithmetic`, `Point`, `Counts`, `PRIME31` and `check_curve` from immutable commit `1df714645aa683cf975bc76fcacb9842cb4780fd`. Its base comprises the first 31 rational x points with canonical ordinates. It independently applies six public scalar weights `1,3,5,7,11,13` to those points, builds two XOR-parity three-sum tables, and finds **natural public-residual-only** weighted signed-free six-point group relations by a *generic* 3+3 point-table search.

The relation matrix has 31 unknown base logarithms. With fixed training seed `0x4d36545241494e31`, **31 naturally discovered odd-parity equations in 106 public training calls** give modular rank **31/31**. Every recovered base logarithm satisfies `[ell_i]G=P_i`. On an independent fixed corpus of **1,024 blinded public Q** (`0x4d36544152474554`), both implementations recover **1,024/1,024** exact original-curve unknown logarithms via **3,717** total public Q+shift membership queries, with **zero** BSGS/rho fallbacks. Each recovered `k` satisfies `[k]G=Q` and the caller-only reference scalar.

The experiment changes **only** how the affine complements `Q-L` are computed: scalar point-by-point inversion versus classic Montgomery batch inversion with all exceptional identity/opposite/doubling branches preserved. A standalone B2 exhaustive parity test compares both oracles on every 64-word image and additional negative targets.

## Matched fully-charged Rust program wall times

**One GitHub-hosted Ubuntu 24.04 runner**, two repetitions per arm in the preregistered A/B, B/A execution order. Both examples compiled before timing. Each measurement includes original-curve certificate, base generation, exhaustive B2 controls, table setup, natural relation training, modular rank, 1,024 public target queries, failed shifts and `[k]G=Q` verification. No target labels enter the oracle.

| Same-host run | Scalar complement A | Batched complement B | Paired A/B speedup |
|---|---:|---:|---:|
| A then B | 43,428.237 ms | **5,812.828 ms** | **7.471×** |
| B then A | 43,510.030 ms | **5,988.724 ms** | **7.265×** |
| Median paired ratio | | | **7.368×** |

For the B arm, query+verification wall was **5,618.953 ms** and **5,796.322 ms**, training **161.822 ms** and **160.477 ms**, table setup **29.755 ms** and **29.632 ms**. Unisolated runner variance remains; this is a two-pair sample, not a population confidence interval.

## Fully counted arithmetic and exact control equality

| Full 1,024-Q query+verification phase | Scalar A | Batched B |
|---|---:|---:|
| Group additions | 110,834,780 | **110,834,780** |
| Fp inversions | 110,831,708 | **102,278** |
| Fp multiplications | 7,093,290,578 | **670,994,789** |
| Correct full target logs | 1,024 | 1,024 |

Training has the same 3,162,785 EC additions in both arms, but batched inversions reduce training Fp inversions from 3,162,648 to **4,908**. Both table setups are identical at 62,465 EC additions, 62,279 Fp inversions and 3,986,383 Fp multiplications.

Exact source-bound workflow [#44295520583](https://github.com/isogeny-labs/crypto/actions/runs/44295520583) on candidate SHA `1cab702112f7d3af371d6b5af349d5a443615a11` passes 40/40 original Rust library tests; 2/2 scalar and 2/2 batched-focused tests; four independently coded Python integer-only original-curve full-transcript replays; and **byte-equivalent algebraic data** across arms: 31 base logs, 31 training equation words, every one of the 1,024 Q/log/six-leaf words, same 3,717 shifts and identical 31-rank decisions. One intermediate pre-fix source CI failed to compile due to a nested mutable Rust borrow; corrected without changing curve, weights, corpus or mathematics.

Source and evidence: [preregistered v2 protocol](PRIME31_WEIGHTED_M6_BATCH_PROTOCOL_20261118.md), [scalar source](examples/prime31_weighted_m6_complete.rs), [batched source](examples/prime31_weighted_m6_batch.rs), [separate batched BigInt verifier](verify_prime31_weighted_m6_batch.py), [committed complete-cost summary JSON](PRIME31_WEIGHTED_M6_BATCH_ABBA_SUMMARY_20261118.json), and [raw four complete 1,024-Q transcripts, independent audits and summary](https://github.com/isogeny-labs/crypto/actions/runs/44295520583/artifacts/14422898426). Provider-reported ZIP was also downloaded and its SHA256 independently checked as `64f26701b748076a335626e7cdc517a0746e51a4a524fd25c62d5792375353cf`; artifact retention 30 days. Signed PCC gate still pending.

## Comparison to the actual research objective

**This is 7.37× faster than a matched complete Rust implementation of the same index-calculus-style six-weight algorithm.** It is *not* a 7.37× improvement over the strongest external solver, nor evidence of non-generic ECDLP complexity. The oracle still enumerates 31³=29,791 generic EC group sums per half and computes over 110 million EC complement queries for 1,024 logs. The group-addition count is unchanged.

Historical reference: [merged #4623](https://github.com/isogeny-labs/crypto/pull/4623) reports a 7.658s 1,024-target generic Dickson31 control in an unrelated CI run, and [merged #4596](https://github.com/isogeny-labs/crypto/pull/4596) cites a separate-run, shared-precompute BSGS result of 170.113ms; these are **not matched cost ratios** to this new 5.8–6.0s execution. The older B31 4+4 weighted parity [#4714](https://github.com/isogeny-labs/crypto/pull/4714) has 7/12 odd relations with much larger raw tables, but no complete solver arm. The mask-only eight-slot [#4716](https://github.com/isogeny-labs/crypto/pull/4716) is ruled out for identical-image subsets by a support theorem; distinct scalar images avoid that particular obstruction.

The exact native B31 six-weight odd-hit rate 31/106 (~29.25%) is compatible with a naive random-sum Poisson odd-parity heuristic (31⁶/n≈0.4133, expected odd rate≈28.12%). This is *not* proof of independent representation counts, asymptotic scaling or special prime-field weakness. The method uses known-public-coefficient training (no leaked Q), but its generic group table is *explicit*, so do not present it as novel cryptanalysis.

## Ranked discriminating experiments to pursue next

1. **Nonenumerative signed-root extractor**: supply these **same frozen public residuals** to existing native RR9/F4/quotient and rootfinding in [#4692](https://github.com/isogeny-labs/crypto/pull/4692), with only public R and slot sets. Require complete B4/B8 positive+negative and repeated/identity cases first. No 3+3 table or target-scalar labels in extraction. Gauge useful relations/rank per complete failed/censored call, not only algebraic witness verification.
2. **Algebraic point-image representation**: test six disjoint low-degree Dickson fibres, one per slot, against six scalar multiples of one fibre. The former increases matrix unknowns to as many as 186 but keeps each slot's low-degree fibre structure; the latter keeps 31 unknowns but may destroy low-degree input equations. Freeze a same-curve B8 and B16 positive/negative rootfinding matrix protocol before choosing the better candidate.
3. **Complete frontier control**: same runner, *identical 1,024 Q*, full cold setup/memory/failed relation work versus best shared BSGS (not just this slower scalar inversion comparator), current strongest prime-field index calculus and optimized negation-map rho; include 0, 0.1 and 1 GAE opportunity prices when applicable.
4. **Adversarial and independent validation**: fresh holdout seeds, adversarial parity-even planted witnesses, field-op replays, modular equation rank rederivation and signed PCC issuance by independent verifier. No leaderboard promotion from unsigned source-only checks.

**Disposition:** the new complete-solver batching improvement is reproducible, but the target of a genuinely cheaper **non-generic** prime-field ECDLP solver remains open. All speed and completeness claims here are scoped to their exact-source Rust diagnostics until signed independent standing is obtained. [pcc:unverified]
