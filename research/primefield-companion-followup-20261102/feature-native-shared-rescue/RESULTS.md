# Shared interval engine — T-1880

Producer observations; independent signed verdict pending. [pcc:unverified]

validation01 passes on its first attempt, including inherited checks. Twenty scalar-stream panels compare 760 delivered coefficients across shared, separate equal-width and separate native-width engines. All values agree. There are 120 rejected-key/borrow controls and 20 interleaved-stream pairs; owner cleanup and bounded batch size are checked.

| Count across 20 panels | Shared original width | Separate equal width | Separate native widths |
|---|---:|---:|---:|
| Polynomial setups | 16 | 32 | 32 |
| Progression setups | 16 | 32 | 32 |
| Evaluation batches | 10147 | 10147 | 9953 |

Sharing halves the two setup counts on these panels. The equal-width comparison isolates sharing: evaluation batches are unchanged. Keeping the original width costs 194 more batches than native sizing across these panels. No total-operation or elapsed-time conclusion follows from these counts.

## Public recovery

| Degree cap | Source calls | Recovered | Shared engines | Warm / cold shares |
|---|---:|---:|---:|---:|
| 8 | 4053 | 271/281 | 0 | 0 / 0 |
| 10 | 3278 | 281/281 | 260 | 0 / 260 |
| 16 | 3228 | 281/281 | 260 | 0 / 260 |
| 20 | 3224 | 281/281 | 260 | 0 / 260 |
| 32 | 3068 | 281/281 | 205 | 0 / 205 |
| 64 | 2881 | 281/281 | 0 | 0 / 0 |

All 1686 target/cap comparisons match the fresh-engine rescue on scalar result, source calls and route. Cap 8 retains ten unresolved targets on each path; the other panels recover all targets. All observed public shares are cold: m=5 and 2m=10 intervals use the direct product path below length 32. Therefore these public fixtures do not demonstrate avoided warm polynomial/progression setup. The larger scalar-stream panels do exercise warm reuse.

Rc introduces an allocation for each original engine and reference-count operations. The previous inline RefCell did not require that Rc allocation. This report does not claim reduced allocation count or peak memory. Coefficient setup/advancement, width-related batches, exact-order checks, source work and final verification remain charged.

```
shared_rescue_panels=20 coefficient_equalities=760 shared_polynomial_setups=16 fixed_width_polynomial_setups=32 native_width_polynomial_setups=32 shared_grid_setups=16 fixed_width_grid_setups=32 native_width_grid_setups=32 shared_evaluation_batches=10147 fixed_width_evaluation_batches=10147 native_width_evaluation_batches=9953 rejected_controls=120 interleaved_stream_pairs=20
shared_rescue_public_cases=281 cap=8 source_calls=4053 recovered=271 route_changes=0 shared_engines=0 warm_shares=0 cold_shares=0 retained_width_sum=0
shared_rescue_public_cases=281 cap=10 source_calls=3278 recovered=281 route_changes=0 shared_engines=260 warm_shares=0 cold_shares=260 retained_width_sum=780
shared_rescue_public_cases=281 cap=16 source_calls=3228 recovered=281 route_changes=0 shared_engines=260 warm_shares=0 cold_shares=260 retained_width_sum=780
shared_rescue_public_cases=281 cap=20 source_calls=3224 recovered=281 route_changes=0 shared_engines=260 warm_shares=0 cold_shares=260 retained_width_sum=780
shared_rescue_public_cases=281 cap=32 source_calls=3068 recovered=281 route_changes=0 shared_engines=205 warm_shares=0 cold_shares=205 retained_width_sum=615
shared_rescue_public_cases=281 cap=64 source_calls=2881 recovered=281 route_changes=0 shared_engines=0 warm_shares=0 cold_shares=0 retained_width_sum=0
```

Compiler: rustc 1.99.1 (58cf8f9c3 2026-10-13); native source pins: 98. PCC-20261107-2184b1 pins the derivation; independent signed verdict pending. No compile/native/render failure, cloud job or timing run.
