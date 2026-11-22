# Prime31: fully charged weighted-M6 versus shared BSGS, plus native mixed-IC frontier

2026-11-18 UTC · candidate [PR #4714](https://github.com/isogeny-labs/crypto/pull/4714) · [prospectively committed protocol](PRIME31_WEIGHTED_M6_SHARED_BSGS_FRONTIER_PROTOCOL_20261118.md) · [durable record](PRIME31_WEIGHTED_M6_SHARED_BSGS_FRONTIER_RESULT_20261118.json)

**Producer-source status [pcc:unverified].** Two source-specific independent GitHub-hosted executions passed. The generic-equivalent weighted-M6 complete Rust solver recovered **1,024/1,024 blinded source-curve logarithms** in *each* of four executions, but cost **20.85–23.32× more than shared-BSGS**, and **1.77–1.88× more than the existing native mixed-4+4 IC** measured once earlier on the respective same runners. Thus the previously measured 7.37× batch-inversion improvement against unbatched M6 does *not* establish a competitive ECDLP solver.

## Unchanged curve, source, corpus and classification

Original ordinary cofactor-one prime-order curve:
`p=2147483647=2^31−1; y²=x³+847514846x+2087444724; n=2147446831; G=(625037715,2100627251)`.

This experiment **reuses**, not rewrites:
- the B31 six-distinct-public-weight (`1,3,5,7,11,13`) existing Rust 3+3 XOR-parity relation and rank/recovery code, with *only* new optional caller-verified corpus input;
- the existing original-curve mixed first-x/Dickson 4+4 **complete Rust index-calculus** program, which generates and solves the 1,024 public Q corpus;
- the original `#4596` Jacobian shared-BSGS, unmodified, at `m=46341,262144,1048576` on those **identical public Q**;
- pinned original source `1df714645aa683cf975bc76fcacb9842cb4780fd` and Rust 1.100.0.

The source Q corpus generator is a one-time **shared fixture**, outside the compared M6/BSGS timers. Its own `setup_ns+descent_total_ns` is a valid single-source native IC timing but **not an ABBA third arm**. Candidate and reference take Q, never caller-only k. Every resulting scalar and point is independently checked against held caller-only answers in the workflow; the candidate and BSGS additionally check `[k]G=Q`. The M6 timer excludes the external caller corpus validation (as does the BSGS per-arm timer); M6 includes its B2 self-check, so accounting is conservative to M6 at a negligible share of this gap.

## Two independent GitHub-hosted ABBA executions

All figures are *milliseconds of fully charged in-process solve*, after compilation, on each original curve; BSGS arm includes cold baby-table construction and all 1,024 public target recoveries. A/B means M6 then shared BSGS, B/A the reverse. **Ratios greater than one mean M6 costs more.**

| Original source runner | Execution order | M6 complete | Best shared BSGS (m=1,048,576) | M6 / BSGS |
|---|---|---:|---:|---:|
| [#44305053128](https://github.com/isogeny-labs/crypto/actions/runs/44305053128) | A then B | 3,359.597 | **144.049** | 23.32× |
| Same runner | B then A | 3,391.541 | **157.859** | 21.48× |
| [#44305062836](https://github.com/isogeny-labs/crypto/actions/runs/44305062836) | A then B | 5,301.732 | **250.565** | 21.16× |
| Same runner | B then A | 5,330.527 | **255.661** | 20.85× |

The 4-ratio median is ~**21.32× M6 slower**. The host-to-host time shift illustrates why historical cross-run timing comparisons must not be presented as matched ratios.

A memory-friendlier reference with **m=46,341** (rather than a million baby entries) still cost only **833.466/849.112 ms** on runner 1 and **1287.283/1277.092 ms** on runner 2, roughly **4× faster** than M6. For m=262,144 BSGS cost 190.080/189.874 and 287.481/282.170 ms.

### Same-runner existing Rust mixed-index-calculus control (secondary)

The unchanged original mixed 4+4 first-x/Dickson Rust solver did a full 61/61 basis solve and returned 1,024/1,024 logs while generating the target corpus. Its **single** whole solve on runner 1 was **1,897.579 ms** (1,096.335 setup + 801.244 descent), and runner 2 **2,841.305 ms** (1,785.605 setup + 1,055.701 descent). The M6 executions cost **1.77–1.88×** this same-host native-IC timing. Because the mixed control was executed only once and before the ABBA paired runs, this is a secondary same-runner comparison rather than a fully randomized three-arm comparison. Both IC methods are explicitly generic-sum enumeration, **not** a non-generic algorithm.

### Correctness and operation diagnostics

- Each M6 run reached rank **31/31** from **31 natural odd relations in 106 public training calls**, then recovered **1,024/1,024** public unknown logarithms using **3,660 shifted public target probes**, no BSGS/rho fallback.
- Each of the three BSGS reference sizes returned 1,024/1,024 matching recovered scalars on each of four arm runs; **6,144 independently checked BSGS answers per workflow**. The source mixed IC also returned 1,024/1,024 per workflow.
- M6's complete online phase charged **109,090,357 elliptic-curve additions**, **56,909 Fp inversions**, **657,872,289 Fp multiplications**. Its 29,791 left and right parity keys remain exact generic three-sum enumerations. On runner 1, online M6 alone cost **3,246.446/3,278.345 ms**; setup and training cost ~15 and ~97 ms. The bottleneck is target-conditioned complement scanning, not the relation matrix.
- With 3,660 target calls plus 106 training calls, **144.049 ms / 3,766 ≈ 38.25 µs per algebraic-oracle call** is an optimistic same-runner total budget to match the fastest BSGS on runner 1. That ignores all algebraic setup, modular LA, verification and other costs and therefore is *necessary but far from sufficient* for a same-yield replacement. As a looser first gate, matching the mixed IC's 1,897.579 ms would permit ~504 µs/call before other overhead. These are derived engineering targets, not measured algebraic-oracle runtimes.

## Frozen raw evidence and validation scope

- [Passing exact-head run #44305053128](https://github.com/isogeny-labs/crypto/actions/runs/44305053128), [full original+M6+BSGS transcripts artifact #14427043131](https://github.com/isogeny-labs/crypto/actions/runs/44305053128/artifacts/14427043131), ZIP SHA-256 `aa61631b9298b2c6acc0d677282d702a2e88489f6af53a09c28f5b0194c58879`.
- [Passing exact-head run #44305062836](https://github.com/isogeny-labs/crypto/actions/runs/44305062836), [full transcripts artifact #14428240804](https://github.com/isogeny-labs/crypto/actions/runs/44305062836/artifacts/14428240804), ZIP SHA-256 `9a3fe05ebe162198c519ca287937607bfe590cac2315977073ad081bbdf109c0`.
- Both source-bound runs tested the existing original Rust library, the focused M6 controls and shared BSGS tests, then verified every reference and candidate answer by a separate **workflow** Python JSON and caller-only scalar comparison. These are **producer-local checks**, not independent signed PCC attestation. GitHub artifacts are retained for 30 days. The SHA-256 values above were computed from connector-downloaded archives, not asserted by the producer's Rust binary.
- The 7.368× previous M6-versus-unbatched-M6 comparison in [PR #4714](https://github.com/isogeny-labs/crypto/pull/4714) was a legitimate narrower engineering result, **not** a win against the best solver.

## Prior leads, exclusions, and the next discriminating experiments

1. **Priority 1 — nonenumerative public-only RR9/F4/quotient extractor.** Continue [#4692](https://github.com/isogeny-labs/crypto/pull/4692), not a second implementation here. Supply the **same 1,024 public Q and 106 training residuals** to original Rust F4 and quotient/rootfinding source. Prove positive and negative membership at B4/B8, including repeated roots, identity and `x_i=x(R)`. Do not feed known ordinates, scalar labels, precomputed three-/four-sum tables or guessed witness coefficients to algebraic extraction. Every completed root must pass full signed group replay. Escalate only after natural relations, growing rank and bounded full-call budgets; timeouts remain *undetermined*, not missed relations. Initial measurement should determine whether <504 µs/call is remotely viable for beating the same-runner mixed IC, with <38 µs/call an optimistic competitive BSGS screen at the observed M6 yield. Do not misrepresent a capped F4 step as completion.
2. **Priority 2 — existence-versus-parity generic control.** The current M6 oracle discards even multiplicity although *any* valid signed witness suffices for a relation. Measure an early-exit witness-seeking variant with the existing Rust 3+3 table, exact duplicate and cancellation controls, and the identical public Q. Compare actual coverage, rank, all-query cost and complete 1,024-Q recovery. This is a **generic-engineering null/positive control** only; it must never be labelled non-generic. Stop if an optimized mixed 4+4 baseline remains faster.
3. **Priority 3 — algebraic factor-base images, not y-re-ranking.** The original first-x versus y-ranked factor base [#4652](https://github.com/isogeny-labs/crypto/pull/4652) was a measured 28–36% loss. Test independent low-degree Dickson x-fibres for six slots against `[1,3,5,7,11,13]` scalar images only if an actual polynomial quotient degree/normal-form budget improves. Slots with different fibres raise unknown log columns (up to 186); record total rank-yield and complete setup. The earlier S4→chained-S3 F4 experiment `S4_S3_CHAIN_TARGET_DEPENDENT_GATE_20261116.md` exhausted its caps; do not rerun presentation-only permutations.
4. **Priority 4 — red-team asymptotic parity theorem.** [#4698](https://github.com/isogeny-labs/crypto/pull/4698) claims `r^{o(1)}` expected classical prime-field ECDLP via a graded parity tensor counter; its report is unsigned and has no end-to-end Rust test. Recheck its cited formal source and complete the independent proof/complexity audit (binary network encoding, bit-pinned witness preservation, group-order elimination, exact non-heuristic rank/yield), then implement an audited Rust B4/B8 network counter and collect *actual* operation-scaling evidence before applying the theorem to this Prime31 baseline. Treat the extraordinary asymptotic statement as a hypothesis until signed independent standing is obtained.
5. **Priority 5 — full frontier and validation.** Before promotion, require same-runner fastest native IC and optimized negation-map rho at **1, 64 and 1,024** targets (setup amortized explicitly), memory caps, calibrated group/field prices 0/0.1/1 GAE, fresh held-out points, cross-host replay and signed PCC. The current result is a correct negative frontier control, not a security improvement.

**Scientific disposition:** A new Rust cohort bridge and controlled complete index-calculus-versus-BSGS comparison are checked in, with both outcomes retained. The existing weighted-M6 method is substantially more expensive than the strongest matched reference despite its prior engineering gain. Fresh, genuinely nonenumerative prime-field relation recovery and a cheaper complete prime-field ECDLP solver remain open.
