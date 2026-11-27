# Round 144: stack-resident Mersenne polynomial power

Parent d91afaaee2f4f96b01d86651c064134bf16ca055 (Round 143). This is an implementation optimization, not a new root-finding algorithm. Sources checked: parent isogeny.rs, round 142/143 local archive and prior native qualification. Shared KB unavailable; direct clone DNS blocked. Local optimized C++ reproduction, not native Rust; native qualification configured separately.

Fixed-size degree 3/4 arrays avoid intermediate Vec allocation and trimming; symmetric squaring computes one product per pair and doubles cross terms. Two-fold Mersenne convolution and fixed-width monic reduction. At most four products per coefficient, so each sum is below 2^124. Rust counts actual coefficient products. Generic fallback unchanged for other moduli and noncanonical input.

## Local task-clock nanoseconds per full operation

AMD EPYC 9V74, pinned CPU, GCC 14.2 and Clang 17, Linux perf_event_open task-clock. Four seeds/compiler, 5 trials per seed, first omitted. 280 paired checksums matched. GCC rerun after adding oracle-only code, original Clang run retained after second attempt timed out. Timed code unchanged. No standalone perf executable or hardware cycles.

| Workload | GCC before | GCC after | GCC ratio | Clang before | Clang after | Clang ratio |
|---|---:|---:|---:|---:|---:|---:|
| Cubic | 6676.65 | 2568.30 | 2.600 | 5884.06 | 2559.91 | 2.299 |
| Quartic | 10080.89 | 6166.98 | 1.635 | 7433.03 | 4088.44 | 1.818 |
| Nonmonic quartic | 12172.79 | 6877.42 | 1.770 | 7772.11 | 4314.48 | 1.801 |
| Polynomial exponentiation | 9202.83 | 5779.74 | 1.592 | 6727.17 | 3617.25 | 1.860 |
| Division polynomial roots | 12659.59 | 8422.87 | 1.503 | 7940.19 | 5137.89 | 1.545 |
| 2-isogeny | 6201.67 | 2721.52 | 2.279 | 5587.72 | 2518.21 | 2.219 |
| 3-isogeny | 12346.04 | 7548.67 | 1.636 | 8110.74 | 5195.96 | 1.561 |

8000 independent full-width polynomial power checks, 60000 product checks, prior powmod oracle and Clang ASan/UBSan passed locally. Production native CI uses both original modules and exact wide reference. Local source and raw CSVs are included in the run's archive. No independent PCC or broad Rust test claim. [pcc:unverified]
