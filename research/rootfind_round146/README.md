# Round 146: exact Mersenne addition chains

Baseline: 4f4b9fc4ee2ed9a9ae004d93b46482d7f9b675d9, PR #5089. Candidate #5098. For B_k=b^(2^k-1), B_(i+j)=B_i^(2^j)*B_j in any commutative quotient ring, including zero divisors. Chain 1,2,3,6,12,15,30,60,61 uses 59/60 squares and 7/8 general products vs width-three 58/59 squares and 22/23 general products. Exact exponents only; fallback unchanged.

Local same-process pinned Intel Xeon Platinum 8573C, GCC 14.2 and Clang 17, Linux perf_event_open task-clock. Four seeds, 9 alternating pairs (discard first), 32 retained pairs/compiler/workload. Medians of paired ratios:

| Workload | GCC before ns | GCC after ns | GCC speedup | Clang speedup |
|---|---:|---:|---:|---:|
| cubic |2234.426|2049.562|1.064|1.355|
| quartic |3311.762|3054.996|1.125|1.020|
| nonmonic quartic |3519.437|3259.295|1.113|1.064|
| powmod |2754.417|2360.537|1.140|0.992|
| psi3 |3897.371|3583.206|1.149|1.085|
| isogeny2 |2362.899|2210.527|1.074|1.310|
| isogeny3 |4008.881|3571.267|1.088|1.043|

Isolated three-way ablation (two seeds, 18 retained triples/compiler/workload): degree-three chain alone 1.247–1.391x; degree-four GCC 1.164–1.194x, Clang 0.987–1.029x. Final sparse x-multiply did not improve consistently and is removed; sparse shifted-linear initialization also rejected. 504 full workload checksum pairs, 320 ablation checksum triples, 10000 independent exact-chain cases, 8000 previous independent fixed power cases, 60000 previous product cases, GCC/Clang oracles and Clang ASan/UBSan passed. Standalone perf absent (no package candidate), hardware cycles errno 2, local rustc absent; C++ reproduction only. Native two-job workflow uses source-exact baseline and candidate and 45000 independent powers. Native timings and broad tests pending. No novelty assertion. Sources: round145 source, report and previous oracle; KB unavailable, direct clone DNS failed. [pcc:unverified]
