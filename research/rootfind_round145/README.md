# Round 145: deferred reduction for degree 3/4 Mersenne polynomial powers

Baseline d28cff7a261cd368efd71809846be0fdd86c8c88 (#5086). Local C++ reproduction of the fixed-size Rust path; native source-exact benchmark configured separately. No external-tool, production Rust or independent PCC speed claim yet.

Pre-negate modulus coefficients once; then accumulate products from monic reduction in the same u128 convolution columns. Fold only when consuming each upper column or emitting a lower coefficient. A column contains at most D convolution and D-1 reduction terms, so for D<=4 the bound is 7*(p-1)^2 < 7*2^122; even the first fold fits u64. For degree 4 this reduces the number of coefficient modular folds from 19 to 7; degree 3 from 11 to 5. Multiplication tally unchanged. Same width-three exponent schedule and fallback paths. Prior art checked: round 144 README, implementation and exact oracle; conventional polynomial long division and Mersenne folding, no novelty claim. Research KB and direct git clone unavailable; local profiling requested.

## Local Linux task-clock, Intel Xeon Platinum 8573C

CPU-pinned, GCC 14.2 and Clang 17, baseline/candidate alternating within the same process on identical input, four seeds, nine pairs per seed, first omitted. 32 retained pairs per compiler/workload; speedup is median of paired ratios, not ratio of separate medians. 504 checksums matched across all pairs including warm-up. All 1008 measurements used perf_event_open software task-clock; standalone perf unavailable.

| Workload | GCC before ns | GCC after ns | GCC speedup | Clang before ns | Clang after ns | Clang speedup |
|---|---:|---:|---:|---:|---:|---:|
| Cubic |2862.360|2281.809|1.295|3242.520|2389.931|1.328|
| Quartic |4891.334|3424.869|1.366|5619.108|3527.303|1.628|
| Nonmonic quartic |5306.418|3498.991|1.352|5927.752|3757.914|1.569|
| Powmod |3981.537|2590.368|1.463|5117.960|2969.813|1.717|
| Psi3 |5116.941|3871.581|1.321|6950.379|4051.708|1.594|
| Isogeny2 |2972.399|2278.756|1.314|3278.586|2519.546|1.343|
| Isogeny3 |5614.898|4126.752|1.388|7074.582|4598.150|1.599|

Independent 8000 power checks, 60000 product checks (the preexisting general multiplication path), previous powmod oracle and Clang ASan/UBSan passed. Exploratory wider-window process measurements were noisy, so no width change retained. Local raw measurements and sources packaged in accompanying run artifact. Native Rust and full repository CI remain open. [pcc:unverified]
