# Prime31 run65: exact fixed-shift autocorrelation and eight-probe support union

**Date:** 2026-11-22 UTC. **Lane:** draft [PR #4901](https://github.com/isogeny-labs/crypto/pull/4901). **Curve:** the *unchanged ordinary cofactor-one prime-field curve* (E/F_p), (p=2^{31}-1=2147483647), (a=847514846), (b=2087444724), (#E=n=2147446831), (G=(625037715,2100627251)). This study does not compute an unknown discrete log.

## Question and method

Runs 55–59 modeled public shifts (Q+[k]G) by uniform independent relation probes. Each marginal is uniform for uniform (Q), but the successive translates are **not independent**. Before changing strides, compute their exact finite-population behavior rather than extrapolate from microbenchmarks.

For each point support \(S\subset E(\mathbb F_p)\), define

$$
I_d(S)=|S\cap(S+[d]G)|,\qquad
U_8(S)=\left|\bigcup_{k=0}^{7}(S-[k]G)\right|.
$$

If \(Q\) is uniform, then \(\Pr(Q,Q+[d]G\in S)=I_d/n\), and the probability of at least one hit in eight consecutive shifts is **exactly** \(U_8/n\). The independent-marginal comparison is \(n[1-(1-|S|/n)^8]\); it is a *model*, not an assertion that the shifts are independent.

The fixed sets are the prior frozen public signed two-sum supports: Dickson depth 10 (509 unsigned base points, 518085 distinct sums including O); the 677-point Dickson + signed (y^7) hybrid (916469 sums including O); and the run60 quotient-rank-visible nonidentity subsets after **100/400/900 synthetic** positive pair rows, of sizes 266/239920/808824. No unknown factor-base indices are recovered, and the synthetic relation rows are not claimed to come from an algebraic oracle.

## Exact full-population results (C++ replay, independently checked lags in Python)

| Frozen support | Size | I1 | I2 | I4 | I8 | I16 | Exact U8 | Independent-model U8 | Relative difference |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Dickson | 518085 | 124 | 124 | 136 | 114 | 96 | 4141238 | 4141181.933 | +0.001354% |
| Hybrid all signed pairs | 916469 | 382 | 486 | 392 | 400 | 306 | 7320436 | 7320809.904 | -0.005107% |
| Hybrid visible at m=100 | 266 | 0 | 0 | 0 | 0 | 0 | 2128 | 2127.999 | +0.000043% |
| Hybrid visible at m=400 | 239920 | 18 | 34 | 36 | 42 | 18 | 1918712 | 1918609.637 | +0.005335% |
| Hybrid visible at m=900 | 808824 | 328 | 356 | 332 | 322 | 204 | 6461726 | 6462068.528 | -0.005301% |

The **pair-lag structure is nonuniform**: e.g. the hybrid full-base I2=486 versus uniform-model expectation (|S|^2/n=391.123), whereas I16=306. However, the exact *eight-query union* departs from the independent-marginal expectation by at most **0.00534%** across these five frozen sets. In this specific bounded 8-shift window, tuning a small shift stride has no plausible meaningful throughput benefit. This is not a proof of independence for long sequences, alternative stride schedules, or future relation oracles.

The exact two-probe hit probability is ((2|S|-I_d)/n); for hybrid full support, changing lag 2 to lag 16 changes that probability by only ((486-306)/n=8.382\times10^{-8}). Thus even visibly different local correlations induce a minute eight-query **hit probability** effect. They do not measure computational rank per hit.

## Provenance and validation

- Hybrid base SHA256 `522747b6cf805c025be4f59d194ec3335a57aff97528fb841cf7c2b4ec2f0162`; Dickson base `fa6a97bb09d2224fb75dfd51aa84fcea0aaa28d4bf08272c2b400827b1ab3f04`.
- The run60 m100/m400/m900 class-label files have SHA256 prefixes `71079fd0`, `034c22b3`, `0137c1c4`; replay checked exact pair census 917335, identity records 677, and all four signed-support sizes before calculating any overlaps.
- **Independent C++ and independent Python affine arithmetic both exactly reproduced all five lag-overlap counts** for each hybrid support and Dickson. These independent checks include full group-law sums, not merely Monte Carlo sampling. The eight-shift unions were computed by C++ only at local freeze; the native Rust gate below independently recomputes the Dickson eight-shift union.
- Reuse original `prime31_algebraic_ic::{Arithmetic,Base,PRIME31}` in [run65 Rust example](../prime31_ic_20261115/native/examples/prime31_run65_shift_correlations.rs) with expected Dickson counts and its complete eight-shift union. [Source-specific workflow](../../.github/workflows/prime31-run65-shift-correlations.yml) uses Rust 1.100.0. **CI was pending at first check-in; do not call it passed unless the job finishes successfully.** No local Cargo compiler was available.
- Independent full eight-shift Python replay was **not completed** because the unoptimized eight-fold modular-inverse loop exceeded its runtime; no independent Python eight-union claim is made. The independent original-Rust control is the intended second implementation for the Dickson union.

## Prior work and novelty scope

This is an exact-correlation control for **this frozen factor base**, not a new index-calculus algorithm. Reused #4901 runs 55–64, including run60's exact group support, run59's quotient-rank visibility, and run64's projective S3 evaluation. Avoid duplicate ownership: [#4809](https://github.com/isogeny-labs/crypto/pull/4809) owns S3 batch-root extraction; [#4812](https://github.com/isogeny-labs/crypto/pull/4812) owns F3/Horner; [#4822](https://github.com/isogeny-labs/crypto/pull/4822) owns RR9/Krylov; [#4692](https://github.com/isogeny-labs/crypto/pull/4692) owns the B8 public-only registered positive/negative corpus. Earlier #4807/#4808 cover W8 collision/coverage; these should be reviewed before generalizing any support-correlation claims.

Literature checked: Semaev (2004), https://eprint.iacr.org/2004/031; Amadori–Pintore–Sala (2018), DOI 10.1016/j.ffa.2018.01.009, including one-relation prime-field prior art; Yokoyama et al. (2020), DOI 10.1515/jmc-2019-0029, conditional statistical lower-bound analysis for the naive Semaev method. The research-KB MCP is absent in this runtime, so in-repo notes and the accessible publications were the fallback; there is no exhaustive novelty claim.

## Ranked next experiments, all on the existing ordinary Prime31 Rust curve

1. **P0 / decisive:** on the frozen **46 positive / 18 negative public-only holdout**, execute the *existing* nonenumerative RR9/Krylov (#4822), 14-variable Horner/F3 (#4812) and S3 (#4809) oracles. Count all negative/timeout/OOM attempts; require genuine unknown-index roots and signed EC witness checks. Never substitute a precomputed pair-sum table and call it non-generic.
2. **P1 / integration:** incorporate each verified blind relation in the existing run59 free-row quotient-rank stop and inhomogeneous scalar anchor, then recover and independently verify unknown target (k) by ([k]G=Q). Count independent rank per query and all failures, not simply raw two-point support hits. Use projective S3 only if it reduces the *actual extractor* cost.
3. **P2 / qualification:** cold 1 and batched 32/128/1024 exact unknown-index logs; measure full factor-base setup, F3/RR9/F4, rank/LA, failed probes, individual recovery, verification, memory and fallback on the same runner against best existing prime-field index calculus, optimized negation BSGS and rho. Retain signed PCC and source/runner receipts.

**Status:** The shifted-support correlation question is answered exactly for the frozen windows. A cheaper complete non-generic Prime31 solver is **not established** by a support calculation; the blind algebraic extraction gate remains open.
