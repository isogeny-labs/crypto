# Prime31 six-weight M6 3+3 parity: full-rank/1,024-target Rust control

**18 November 2026. Source-bound native Rust and separately authored BigInt arithmetic replay PASS; signed independent PCC remains [pcc:unverified].**

## What is implemented

- Six different public scalar-weighted copies of the same B31 first-rational-x basis on the **original ordinary cofactor-one curve** over 2^31−1. This is not an altered weak curve or a smooth-order substitution.
- The actual native Rust `prime31_algebraic_ic::Arithmetic` and `PRIME31` source pinned to `1df714645aa683cf975bc76fcacb9842cb4780fd`, not a new Python solver.
- A genuine 3+3 group-algebra **XOR-parity** extractor using group additions and group-point hash lookup. The arithmetic oracle receives *only the public residual point*; there are no known leaves or caller secrets in the oracle.
- Independent 31-variable modular Gaussian elimination, full-rank base-log recovery, every base-point verification; target-specific 64-shift signed word extraction and independent verification of up to 1,024 blinded original-curve discrete logarithms.
- Exact B2 parity support test, a second separately authored Python big-integer field and elliptic-curve implementation to replay the full recorded transcript, rejection of inconsistent rows, and explicit retention of truncated/censored targets.

**Classification:** this uses **generic** elliptic-curve point-sum enumeration. It is a *complete index-calculus-style pipeline control* if it reaches rank31 and 1,024/1,024. It is **not** yet a genuinely non-generic prime-field ECDLP attack. Native arithmetic counts are neither calibrated GAE nor a validated relative-speed claim.

## Measured results and evidence

Original scalar-complement run [GitHub Actions #44294687054](https://github.com/isogeny-labs/crypto/actions/runs/44294687054), source `ca48659a101447b6251661300358dcfe45177c85`, pinned original Rust `1df714645aa683cf975bc76fcacb9842cb4780fd`, complete **success**. 40/40 library tests, 2/2 focused tests, all 1,024 target scalar checks and separate integer-only Python arithmetic replay PASS. The source and protocol were frozen with distinct seeds before this observation. Raw [source-specific complete transcript and independent replay](https://github.com/isogeny-labs/crypto/actions/runs/44294687054/artifacts/14422409269), 30-day artifact retention, provider ZIP SHA256 `505ccac5bfa6e759d383583a948c6e591d1b86665aef256fc92e68814f493682` (not rehashed by independent PCC).

| Metric | Current result |
|---|---|
| Original curve / generator | pinned exact cofactor-one Prime31 |
| Raw ordered three-slot words per half | 31³ = 29,791 (combinatorial identity) |
| Distinct scalar weights | 1,3,5,7,11,13 |
| Training cap | 512 public random-style residuals |
| Target cohort | 1,024 public Q; 64 shifts per Q |
| Full rank / successful logs | **31/31**, **1,024/1,024** original-curve unknown logs, no generic fallback |
| Original-curve arithmetic replay | **PASS** separately coded Python BigInt checker: 31 training rows, 31 base logs and 1,024 unknown targets; no signed PCC |
| Training natural odd residuals | **31 / 106** calls, measured parity hit rate 29.25% (fixed deterministic corpus) |
| Public target queries | **3,717** including shifts for 1,024 distinct Q, or 3.6299 calls/target |
| Whole-program wall (producer source-run) | **29,082.809 ms**, including original curve certificate, base generation, exhaustive controls, setup, training, target recovery and verification |
| Three-sum halves | 29,791 raw ordered words each, 62,465 group additions in reported candidate table setup |
| Training counters | 3,162,785 EC additions; 3,162,648 Fp inversions; 202,412,621 Fp multiplications; 807.170 ms |
| Candidate table setup | 62,465 EC additions; 62,279 inversions; 3,986,383 Fp multiplications; 21.466 ms |
| Same-host strongest IC / shared BSGS / negation rho | **Not measured in this experiment** |
| Signed PCC standing | **Unverified** (source-specific CI and producer-independent arithmetic replay are not a signed independent validation verdict) |

Workflow: [prime31-weighted-m6-complete.yml](../../../.github/workflows/prime31-weighted-m6-complete.yml).
Source: [examples/prime31_weighted_m6_complete.rs](examples/prime31_weighted_m6_complete.rs).
Independent arithmetic: [verify_prime31_weighted_m6_complete.py](verify_prime31_weighted_m6_complete.py).
Frozen protocol: [PRIME31_WEIGHTED_M6_COMPLETE_PROTOCOL_20261118.md](PRIME31_WEIGHTED_M6_COMPLETE_PROTOCOL_20261118.md).
Open research lane: [crypto PR #4714](https://github.com/isogeny-labs/crypto/pull/4714).

## Prior-lead assessment

See protocol for primary-source links and detailed distinctions. This control follows #4714 weighted eight-slot parity, #4716 subset-mask support failure, the previous *anchored signed* M6 F4 test, and the merged #4623 1,024-target original-curve control. It must not be substituted for the separate genuine algebraic extractor active in #4692. Partial F4 / isogeny / point-arithmetic speedups are not complete ECDLP breakthroughs. #4698 is a paper-level unvalidated asymptotic claim, not a measured practical Prime31 solver.

## Decisive next steps

1. **Completed for v1:** source-specific Rust and BigInt replay passed and retained all natural relation/target rows. Confirm signed PCC independently before adding to a scored benchmark or leaderboard.
2. If rank 31 but targets are censored, analyze per-shift parity yield and test alternative **predeclared** public target-shift schedules as distinct follow-ups. Do not silently retune the frozen experiment.
3. Port the proven high-yield six-weight *membership predicate* to the active RR9/F4 / shared-coefficient solver. A real non-generic result needs an algebraic extraction without generic EC 3+3 enumeration; repeated/permuted/identity/sign branches must be checked in both positive and negative controls.
4. Require end-to-end 1,024 unknown logs, exact native rank, failed-call accounting, setup and memory, and a same-runner benchmark against the strongest mature prime-field IC, shared BSGS and optimized rho. Signed independent validation must precede leaderboards.

No theoretical speedup or security implication is inferred from the reduction of table arity alone.

## Interpretation after first execution

The six-weight B31 experiment unexpectedly reaches full-rank after only 106 training calls, then recovers every one of 1,024 blinded original-curve scalars in 3,717 total shifted public queries. This is a **new successful complete Rust index-calculus-style control**, with reproducible source-bound results **[pcc:unverified]**. The natural parity rate (31/106 = 29.245%) is close to, but does not establish, the independent-Poisson approximation for 31^6/n ≈ 0.41328: odd rate ≈28.12%; more curves and independent seeds would be needed before claiming any distribution law.

At 29.083s for 1,024 unknown logs, this is **not the requested cheaper frontier solver**: the previous archived generic Dickson31 control #4623 reported 7.658s in an unrelated GitHub execution, while historical large-table shared BSGS reported 170ms. Those historical data are **not same-host matched comparisons**, but they give a clear warning against a practical advantage claim. Each public parity complement still performs ~29k ordinary affine group additions per shift; hundreds of millions of modular field operations dominate. A second **prospectively frozen** experiment batches those inversions without changing the mathematics or corpora: [Montgomery batch v2 protocol](PRIME31_WEIGHTED_M6_BATCH_PROTOCOL_20261118.md).
