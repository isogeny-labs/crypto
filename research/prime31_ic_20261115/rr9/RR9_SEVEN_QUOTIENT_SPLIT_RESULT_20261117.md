# Prime31 seven-of-eight RR9 quotient: exact split algebra, source-specific Rust result

**2026-11-17 UTC · PR #4581 · bounded algebraic/cost falsification · producer [pcc:unverified].**

## 1. Finding

**The seven-of-eight B8 RR9 coefficient quotient is exactly a split 128-point algebra on all three frozen public residuals.** The existing seven-of-eight sparse-F4/quotient suggestion is therefore **not itself a way to avoid signed-support branches**: it represents the same 128 first-seven sign choices in algebraic form. A clever quotient implementation might still be faster for some inputs, but quotient dimension 128 is not evidence for a nonenumerative or asymptotically improved index-calculus relation extractor.

For public R=(r,s), the signed chart is F(X,Y)=(X-r)A(X)+(Y+s)V(X), with A=a0+a1X+a2X²+a3X³, V=X³+v2X²+v1X+v0. Its monic degree-eight norm is

M(X)=(X²+rX+r²+a)V²-(X-r)A²-2sAV.

For any base point (x_i,y_i) with x_i!=r and y_i!=0,

-(x_i-r)M(x_i) = L_i^+ L_i^-,
L_i^± = (x_i-r)A(x_i)+(s±y_i)V(x_i).

Hence the ideal of the *first seven factor-root equations* is generated (up to invertible scalars) by q_i=L_i^+L_i^-, i=0,...,6. Its zero set over the algebraic closure is the union over 2^7 signed choices of the affine intersections defined by L_i^{epsilon_i}=0. For these three *concrete* public residuals, each 7-by-7 signed coefficient matrix is invertible, and its solution is distinct from every other sign choice. In particular, at any solution the unselected factor of every q_i is nonzero, or else two signed branches would share that solution. The Jacobian of the seven q_i then has full rank seven. All 128 intersections are rational and reduced; there are no other points because the product equations enumerate those 128 linear intersections. Thus by the finite reduced affine algebra/Chinese remainder theorem,

**Fp[a0,a1,a2,a3,v0,v1,v2]/(q_0,...,q_6) ≅ Fp^128.**

This is a direct *instance-specific* algebraic statement, not a claim that every public residual or the full eight-equation RR9 chart has this quotient, nor a lower bound against every symbolic factorization/F4 algorithm. Singular rows, x_i=r, and B31 repeated-root strata need separate treatment.

## 2. Original-prime31 executable verification

**Curve:** p=2^31−1=2147483647, a=847514846, b=2087444724, n=2147446831 (prime cofactor-one), generator (625037715,2100627251). B8 x=[0,1,2,3,5,6,8,9]. Existing native Rust Arithmetic supplies curve certification, canonical y square roots, on-curve checks and *independent original group-law replay*. New code performs modular 7x7 Gaussian elimination, checks every first-seven equation via independently expanded *unfactored* M, and evaluates the eighth two-factor selector for all 128 charts.

The frozen positive and two negative residuals are unchanged from #4548/#4575. No positive targets were selected after seeing results.

| Public R | Full-rank 7x7 signed systems | Distinct solutions | Unfactored first-seven M=0 checks | Eighth-selected charts | Diagnostic Rust elapsed |
|---|---:|---:|---:|---:|---:|
| (1911579686,548820096), known-positive | 128/128 | 128 | 896/896 | **1**, independent group replay | 0.443992 ms |
| (625037715,2100627251), negative G | 128/128 | 128 | 896/896 | **0** | 0.437080 ms |
| (1923055775,307731458), negative #2 | 128/128 | 128 | 896/896 | **0** | 0.419025 ms |

No double-sign ambiguity was found. The sole positive chart has first-seven mask 127, eighth positive, coefficients (455835358,518652820,754716693,932383545,62510441,1884059825,1142049404); native Rust independently sums the eight selected points to the frozen public R. All 2,688 first-seven norm checks pass. The eighth-selector identity is independently verified against the unfactored M on all 384 candidate charts.

**Exact-source PASS:** [pull-request run #44138009239](https://github.com/isogeny-labs/crypto/actions/runs/44138009239) and [push run #44137999498](https://github.com/isogeny-labs/crypto/actions/runs/44137999498), pinned Rust 1.100.0 over original native arithmetic from immutable commit 1df714645aa683cf975bc76fcacb9842cb4780fd plus proposed example. Each passes **40/40 native library tests**, **1/1 new example test**, release JSON assertions. Raw [artifact #14353560993](https://github.com/isogeny-labs/crypto/actions/runs/44138009239/artifacts/14353560993), GitHub SHA-256 digest **6b1aa75e052fa93d6ea5c51c156a3cef5cac02c28f59ab8500cdcae0d12fe673**; downloaded ZIP independently hashed with exact matching digest. Raw JSON is additionally copied unchanged to [versioned repository data](RR9_SEVEN_QUOTIENT_SPLIT_RAW_20261117.json).

The three 0.42–0.44ms figures are single-run, known-base **enumerative diagnostic** times. They are **not** public-only nonsign algebraic extraction times, not isolated-host confidence intervals, and not an end-to-end ECDLP measurement. The example deliberately enumerates 128 signs to audit the ideal and is marked \`is_public_only_nonenumerative_extraction=false\` and \`complete_unknown_prime31_logs=0\`. No independent signed PCC exists.

## 3. Prior leads and avoidance of duplicate effort

- #4539 and #4541: seven-chart polynomial M and all-multiplicity radical gate. #4545/#4546: signed-linear fibres, exceptional residual conditions. #4547: supplied-seven-anchor completion generic-equivalent, plus low-degree all-sign interpolation obstruction. **This run differs:** it audits the *seven polynomial equations as an affine quotient*, not a supplied support.
- #4548/#4549: public-only dense D5/D6 Macaulay stalls with generic Hilbert dimensions 56/28. #4550: projective D9 exact membership theorem, positive witness only. No D9 negative unit certificates yet. #4556/#4558: public B8 barycentric extractor still enumerates 256 signs. #4575: generic B8 4+4 MITM is much cheaper for that membership component.
- #4513/#4532: complete index-calculus/BSGS same 1,024-Q historical panel, mixed IC **2,221.474ms** and shared BSGS **170.113ms** (producer-level, **not rerun**).
- [crypto-autoresearcher #3301](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3301): lead atlas and generic pair-table classification; [#3305](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3305): Semaev alternatives / errata. [cryptanalysis #1536](https://github.com/isogeny-labs/cryptanalysis/pull/1536): matched same-base vs auxiliary/random-base cost controls. Do not promote this sign-equivalent quotient as a new lead already cleared by those audits.
- Primary literature: Petit–Kosters–Messeng, *Algebraic Approaches for the ECDLP over Prime Fields* (PKC 2016, DOI 10.1007/978-3-662-49387-8_1); Amadori–Pintore–Sala, *On the DLP for Prime-Field Elliptic Curves* (FFA 2018, DOI 10.1016/j.ffa.2018.01.009); Kudo et al., *Acceleration of Index Calculus... and its Limitation* (CANS 2018, DOI 10.1007/978-3-030-00434-7_19). Original prime-field IC and hybrid Gröbner methods predate this. Research-KB unavailable in connector-only runtime; named-repo/primary fallback is bounded. No novelty priority claim.

## 4. Decision and prioritized next experiment queue

1. **Retire naive first-seven B8 quadratic quotient as an independent nonenumerative-speed lead.** Keep it as an exact algebraic regression oracle and to benchmark a fully optimized Rust quotient implementation, if needed; certify same 128 roots and measure setup and memory against 128 direct sign-linear solves.
2. **Change the algebra, not the representation:** build a public-R-only sign-free *full-eight-equation* sparse D9 membership/extraction solver using existing Rust F4/quotient, with positive coefficient recovery and replayable negative unit certificates on these same B8 fixtures. Its Macaulay map has 11,440 columns and 27,456 input rows (#4550); do not allocate a blind 1-GiB dense matrix. Preregister block-Krylov/structured sparse strategy, field-operation counting, and 1-GiB guard; randomness cannot certify a negative without replayable proof.
3. **B31 all-multiplicity gate:** implement/support test \`M/gcd(M,M') | H_B\` for unknown coefficients and natural public residuals, including x(R) in the factor base, repeated leaf coordinates and orientation. Compare actual Dickson B31 against same-size random x bases using #1536's protocols. B8 root-factor diagrams cannot be extrapolated to B31.
4. **Complete index-calculus criterion:** recover blind unknown logarithms on the fixed p=2^31−1 *non-weak* prime-order curve with setup, every failed relation attempt, matrix rank, individual-log recovery, memory and wall time charged; one matched Rust cohort versus 1,024-target shared BSGS and optimized rho, plus available external IC comparisons. Require independently replayed scalars and signed PCC before leaderboard promotion.

**Conclusion at experimental scope:** the first-seven quotient is split and sign-enumeration equivalent. The productive target remains a genuinely sign-independent complete B31 extractor; nothing here establishes a cheaper complete ECDLP solver.
