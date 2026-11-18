# Prime31 six-weight M6 3+3 parity: full-rank/1,024-target Rust control

**18 November 2026. Status: native exact-source CI in progress. [pcc:unverified]**

## What is implemented

- Six different public scalar-weighted copies of the same B31 first-rational-x basis on the **original ordinary cofactor-one curve** over 2^31−1. This is not an altered weak curve or a smooth-order substitution.
- The actual native Rust `prime31_algebraic_ic::Arithmetic` and `PRIME31` source pinned to `1df714645aa683cf975bc76fcacb9842cb4780fd`, not a new Python solver.
- A genuine 3+3 group-algebra **XOR-parity** extractor using group additions and group-point hash lookup. The arithmetic oracle receives *only the public residual point*; there are no known leaves or caller secrets in the oracle.
- Independent 31-variable modular Gaussian elimination, full-rank base-log recovery, every base-point verification; target-specific 64-shift signed word extraction and independent verification of up to 1,024 blinded original-curve discrete logarithms.
- Exact B2 parity support test, a second separately authored Python big-integer field and elliptic-curve implementation to replay the full recorded transcript, rejection of inconsistent rows, and explicit retention of truncated/censored targets.

**Classification:** this uses **generic** elliptic-curve point-sum enumeration. It is a *complete index-calculus-style pipeline control* if it reaches rank31 and 1,024/1,024. It is **not** yet a genuinely non-generic prime-field ECDLP attack. Native arithmetic counts are neither calibrated GAE nor a validated relative-speed claim.

## Measured results and evidence

Initial evidence must be populated **only from the source-bound GitHub Actions run and replay**. The source and protocol are frozen with distinct seeds and cannot be tuned based on results.

| Metric | Current result |
|---|---|
| Original curve / generator | pinned exact cofactor-one Prime31 |
| Raw ordered three-slot words per half | 31³ = 29,791 (combinatorial identity) |
| Distinct scalar weights | 1,3,5,7,11,13 |
| Training cap | 512 public random-style residuals |
| Target cohort | 1,024 public Q; 64 shifts per Q |
| Full rank / successful logs | **Pending exact-source native execution** |
| Original-curve arithmetic replay | **Pending independent-run evidence** |
| Same-host strongest IC / shared BSGS / negation rho | **Not measured in this experiment** |
| Signed PCC standing | **Unverified** |

Workflow: [prime31-weighted-m6-complete.yml](../../../.github/workflows/prime31-weighted-m6-complete.yml).
Source: [examples/prime31_weighted_m6_complete.rs](examples/prime31_weighted_m6_complete.rs).
Independent arithmetic: [verify_prime31_weighted_m6_complete.py](verify_prime31_weighted_m6_complete.py).
Frozen protocol: [PRIME31_WEIGHTED_M6_COMPLETE_PROTOCOL_20261118.md](PRIME31_WEIGHTED_M6_COMPLETE_PROTOCOL_20261118.md).
Open research lane: [crypto PR #4714](https://github.com/isogeny-labs/crypto/pull/4714).

## Prior-lead assessment

See protocol for primary-source links and detailed distinctions. This control follows #4714 weighted eight-slot parity, #4716 subset-mask support failure, the previous *anchored signed* M6 F4 test, and the merged #4623 1,024-target original-curve control. It must not be substituted for the separate genuine algebraic extractor active in #4692. Partial F4 / isogeny / point-arithmetic speedups are not complete ECDLP breakthroughs. #4698 is a paper-level unvalidated asymptotic claim, not a measured practical Prime31 solver.

## Decisive next steps

1. Run the exact-source Rust candidate and independent BigInt replay; retain all unrecognized, even parity, censored, failed-rank and failed-target cases as observed.
2. If rank 31 but targets are censored, analyze per-shift parity yield and test alternative **predeclared** public target-shift schedules as distinct follow-ups. Do not silently retune the frozen experiment.
3. Port the proven high-yield six-weight *membership predicate* to the active RR9/F4 / shared-coefficient solver. A real non-generic result needs an algebraic extraction without generic EC 3+3 enumeration; repeated/permuted/identity/sign branches must be checked in both positive and negative controls.
4. Require end-to-end 1,024 unknown logs, exact native rank, failed-call accounting, setup and memory, and a same-runner benchmark against the strongest mature prime-field IC, shared BSGS and optimized rho. Signed independent validation must precede leaderboards.

No theoretical speedup or security implication is inferred from the reduction of table arity alone.
