# Prime31 B8 signed weight geometry: exact collision census and paired natural-target hit control

**2026-11-19 UTC.** Source-specific native Rust 1.100.0 CI [PASS #44515210069](https://github.com/isogeny-labs/crypto/actions/runs/44515210069), job 135529356227, source/workflow commit `2a98e0cf52faa7fca1f5a6f7d69f46be70a1ea0d`. Candidate PR [#4805](https://github.com/isogeny-labs/crypto/pull/4805). **Not an F4 unknown-index extractor; not a non-generic complete ECDLP solver. [pcc:unverified]**

## Exact setup

Reuses original `prime31-algebraic-ic` Rust `Arithmetic` and original Prime31 fixture pinned to `1df714645aa683cf975bc76fcacb9842cb4780fd`: `p=2147483647=2^31-1`, `E: y²=x³+847514846x+2087444724`, prime group order `n=2147446831`, cofactor one, `G=(625037715,2100627251)`. Same canonical first-rational-x31 factor-base points and four overlapping eight-point windows as [#4728](https://github.com/isogeny-labs/crypto/pull/4728). Old weights `[1,3,5,7,11,13,17,19]`; alternative `[1,2,4,8,16,32,64,128]`. Each position has 8 x choices and 2 signs. The reference weight family is the already established generic 4+4 oracle; the alternative is a proposed *geometry/yield control*, not a security reduction.

## New mathematical result: the old 799-word half-set deficit is entirely formal in each tested window

For a half-word `u` with four signed leaf choices define its integer signature `c_i(u)=Σ_{j: i_j=i} ε_j w_j ∈ Z`, `i=0..7`. If two signed words have identical signatures, their EC group sums are equal for *every* curve and base. The converse need not hold. Exhaustive, independently constructed pure-integer enumeration of all 65,536 half-words for each half predicts:

| Per four-slot half | Historical weights | Powers-of-two |
|---|---:|---:|
| Words (ordered) | 65,536 | 65,536 |
| Distinct integer signatures | **64,737** | **65,536** |
| Unordered formally colliding word pairs | 904 | 0 |
| Maximum signature-fiber size | 16 | 1 |

Thus the 799-word *unique-signature deficit*, historically reported as 64,737 actual EC sums per old half, is explained by coefficient identities rather than unexplained elliptic-group collisions. This is an exact finite combinatorial observation, not a lower bound on generic algebraic extraction. The 904 pairs count all within-fiber unordered word pairs, including a 16-word fiber. On the old weights, all four actual curve windows have **64,737 distinct sums** on each half, demonstrating no extra accidental EC collision for these samples. For power-of-two weights, actual distinct half sums in windows 0,2,3 are **65,536**, but **window 1 has 65,496** (40 additional finite-group coincidences among distinct integer signatures). All 64 *unsigned* slot x images are distinct per arm/window, retaining the disjoint-image precondition for RR9 slot resultants on this fixture.

The formal 4+4 signature product rises from `64737²=4190879169` to `65536²=4294967296`, a 2.4235% reduction in old formal pair capacity. This **does not** imply a 2.4235% target-coverage increase; full eight-slot collisions and target intersections are different quantities.

## Source-bound actual original-curve native Rust paired test

The native Rust run used **64 deterministic natural public target points per window**, 256 matched public points total per weight arm, generated from the same fixed SplitMix64 state/curve generator. Every membership test received only the *public* point and public weighted slot lists, not the hidden scalar. Its enumerative generic 4+4 tables are a **held-out grading control**, *not* the candidate F4 extractor. No whole-log recovery took place. The run completed on one GitHub runner (old arm first within each window). Results:

| Window | Old hits / 64 | New hits / 64 | Both | Old-only | New-only | Neither | New half EC sums |
|---|---:|---:|---:|---:|---:|---:|---:|
| 0 | 52 | 58 | 46 | 6 | 12 | 0 | 65,536 |
| 1 | 53 | 58 | 49 | 4 | 9 | 2 | 65,496 |
| 2 | 50 | 51 | 39 | 11 | 12 | 2 | 65,536 |
| 3 | 50 | 59 | 47 | 3 | 12 | 2 | 65,536 |
| **All 256** | **205** | **226** | **181** | **24** | **45** | **6** | Window-specific |

The proposed alternative improves the observed natural-public **generic grader hit fraction** on this one fixed corpus from **80.08%** to **88.28%**, a difference of **+21 hits / 256** (+8.20 percentage points; 226/205 = 1.1024× the hit count). The matched discordances are 45 new-only versus 24 old-only (two-sided *descriptive* exact paired-binomial/McNemar p ≈ 0.01543); the same pilot motivated the candidate and is **not an independently preregistered randomization trial or out-of-sample proof of general superiority**. These are observed deterministic finite-panel counts, not a demonstrated improvement in non-generic extraction, target recovery, curve scaling or complete ECDLP cost.

Native measured all-window grader build+probe totals (not A/B/A paired times, and not a full solver): old **1,899.287 ms**, new **1,757.860 ms**, a producer ratio of **1.080×** with order confounding and early-exit timing; counted EC additions: old **9,876,680**, new **9,114,793**. This is useful only for grader engineering, not the required non-generic ECDLP comparison. No group-add time is claimed as algebraic F4 extraction.

## Evidence, tests, reproducibility

- [Frozen prospective protocol](W8_WEIGHT_SIGNATURE_AUDIT_PROTOCOL_20261119.md).
- [Reused original-crate Rust implementation](../native/examples/prime31_w8_weight_signature_audit.rs) with two mathematical and original-curve tests.
- [Source-bound GitHub Actions #44515210069](https://github.com/isogeny-labs/crypto/actions/runs/44515210069): original native library **40/40**, focused example **2/2**, release 256-target execution and `jq` assertions passed. Tested original arithmetic commit `1df714645aa683cf975bc76fcacb9842cb4780fd`, example/workflow commit `2a98e0cf52faa7fca1f5a6f7d69f46be70a1ea0d`.
- [Exact raw JSON artifact #14525571822](https://github.com/isogeny-labs/crypto/actions/runs/44515210069/artifacts/14525571822); raw JSON SHA-256 `8430535e90e731a5edfab09959b4f052f41146e473d4abf8529d1fb036f284b3`, artifact ZIP digest `ee5b9984e49f4ca9917265603b1c2f5a2b29eed68a426665e6965f66de56083a`.
- [Machine-readable frozen summary](W8_WEIGHT_SIGNATURE_AUDIT_SUMMARY_20261119.json); timestamped [session log](../../../docs/session-logs/2026-11-19-prime31-w8-weight-audit.md). Independent pure-integer signature calculation agrees; a separately signed third-party PCC does **not** yet exist.
- Repo-global Workflow files parse job failed before executing the parser because it could not pull inherited artifact-store data (CI run #44515209875); the isolated source-bound experiment itself passed.

## Previous leads and searched territory

1. [#4728](https://github.com/isogeny-labs/crypto/pull/4728): original position-weighted B8 group-sum grading, four window rank31/1024 complete **generic** Rust pipeline, 64,737 half sums; directly reused.
2. [#4803](https://github.com/isogeny-labs/crypto/pull/4803): exact unweighted signed inverse-cancellation support 187,137 for eight x choices; its unweighted obstruction is not applicable to the position-weighted system here.
3. [#4722](https://github.com/isogeny-labs/crypto/pull/4722): earlier M6 formal-collision/yield ceiling; the present experiment checks the eight-position four-half structure instead.
4. [#4692](https://github.com/isogeny-labs/crypto/pull/4692) owns genuinely public-only RR9/F4 or remainder-tower unknown-index extraction. This work deliberately **does not** duplicate that implementation or pass the grader witness to it.
5. [#4774](https://github.com/isogeny-labs/crypto/pull/4774), [#4793](https://github.com/isogeny-labs/crypto/pull/4793), [#4532](https://github.com/isogeny-labs/crypto/pull/4532) cover full generic rank31 accounting and faster shared BSGS; historical optimized `2^20`-baby shared BSGS completed 1,024 prime31 logs in 170.113 ms in a separate run, **not** a new paired comparison. This pilot's 1.8s generic-grader-only panel cannot establish ECDLP advantage.
6. `crypto-autoresearcher` SA-04/SA-18 negative norm/determinant experiments and the cross-repo search were checked. Shared research-KB MCP unavailable in this runtime; this is a bounded GitHub-original-source/literature audit, not an exhaustive novelty claim.
7. Published prime-field index-calculus context: Petit–Kosters–Messeng, *PKC 2016* (DOI [10.1007/978-3-662-49387-8_1](https://doi.org/10.1007/978-3-662-49387-8_1)); Amadori–Pintore–Sala, *Finite Fields Appl.* 51 (2018) (DOI [10.1016/j.ffa.2018.01.009](https://doi.org/10.1016/j.ffa.2018.01.009)).

## Next experiments in priority order

1. **Frozen unseen natural-target validation of new weights, with disjointness and anomalies.** First diagnose W1's 40 accidental group coincidences (identify witness pairs modulo group order using out-of-band EC grading; compare whether some chosen eight base points are related by short scalar multiples). Register a fresh target seed and at least 1,024 held-out per window; paired old/new on same public points and randomized AB/BA arm order. Compare confidence intervals and full eight-slot *distinct target support*, not merely four-half cardinalities. No hidden scalar provided to the candidate.
2. **Unknown-index RR9/F4 gate**, coordinated with #4692: old and new slot-specific weighted x lists, with no witness, scalar or group-sum table during extraction; verify equivalence to held-out grader on the frozen **46-positive/18-negative B8** public-only panel, and record all misses, memory, F4 Gröbner degree, field ops, and complete root/sign verification. Weight choice may worsen algebraic solving even if it reduces formal collisions.
3. **Reused complete Rust rank31 solver.** Only after non-generic extraction genuinely passes, replace #4728's grading oracle, solve **1, 64 and 1,024** unknown public logarithms on unchanged p31 with complete setup/rank/failed queries/recovery and all verification charged; compare to same-runner best original-code index-calculus, optimized cached shared BSGS and optimized negation-map rho. Historical 170.113ms BSGS is a frontier sensitivity reference from a different runner, not a valid ratio.
4. **Growing nonweak ordinary cofactor-one prime-field curve ladder** with independent signed PCC and exponent/cost scaling. Avoid deliberately weak curves or same-seed benchmark overfitting.

**Standing:** Original-source Rust verification of exact W8 formal collisions and a measured **generic-grading natural-hit improvement on the registered 256-target cohort**. No cheaper complete non-generic prime-field ECDLP solver has been demonstrated.
