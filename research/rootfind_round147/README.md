# Round 147 — fixed-width arithmetic for general primes

[pcc:unverified] Local C++ algorithmic reproduction only. Native Rust qualification and full-repository checks are outstanding.

Retain fixed-width monic degree 3/4 exponentiation over odd p <= 2^61-1; existing Mersenne path dispatches first. Use symmetric squaring, deferred convolution/reduction, negated modulus coefficients and first-set-bit initialization. For D<=4 the worst accumulator has at most seven products: 7(p-1)^2 < 2^63 for p<=2^30 and <2^125 for p<=2^61-1. The small variant therefore uses u64; the large variant uses u128. Stack-resident pseudo-GCD was tested separately and rejected (no consistent end-to-end gain).

## Local measurement

AMD EPYC 9V74, GCC 14.2 and Clang 17, O3 native, CPU pinned, Linux perf_event_open software task-clock. Standalone perf, hardware cycles and rustc unavailable in local runtime. Four seeds, seven alternating A/B trials per seed, first pair discarded: 24 retained paired ratios per compiler/prime/workload/experiment. Across both experiments 7,840 raw timings and 3,920 matching A/B checksums. Results below are paired median speedups, GCC/Clang.

| p | Polynomial exponentiation | Cubic roots | Quartic roots | Complete 3-isogeny |
|---|---:|---:|---:|---:|
| 65521 | 3.150 / 4.683 | 3.417 / 3.558 | 2.686 / 3.596 | 2.416 / 3.152 |
| 1000000007 | 3.212 / 4.585 | 3.536 / 3.686 | 2.919 / 3.921 | 2.414 / 3.246 |
| 4294967291 | 2.694 / 2.766 | 2.367 / 2.673 | 2.414 / 2.570 | 2.146 / 2.172 |
| 2305843009213693921 | 3.127 / 3.182 | 2.468 / 2.693 | 2.835 / 2.833 | 2.435 / 2.515 |

Across all 56 general-prime compiler/workload combinations, 2.146–4.683x. Mersenne results excluded from improvement claim; dispatch unchanged. No SOTA claim or leaderboard promotion. Local artifacts are in run's downloadable archive; native workflow uses exact parent/candidate Rust.

## Correctness

11,000 independent exact convolution/long-division exponentiation cases, 5,500 baseline root comparisons, 2,000 exhaustive small-field root checks; GCC, Clang and Clang ASan/UBSan passed. Native Rust helper includes additional independently reduced polynomial-power oracle and two qualification jobs. Source-exact local Rust not available. No signed PCC receipt or external-tool comparison. 

Sources checked: previous round 146 reproduction and prior production fixed_mersenne_product/fixed_mersenne_pow/powmod. Shared research KB unavailable; direct clone DNS failed, GitHub connector used instead. This is a bounded arithmetic engineering extension, not a novelty claim.
