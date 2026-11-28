# Source-built 14-bit single-curve degree-32 panel — result — 2026-11-25

## What was run

Binary `1ae5170f9b3362f75a636909e1f5d01fb0ead0fd4886627d6012e2b9de9d8d08`
(build receipt `SOURCE_BUILD_RECEIPT_PRIME_INTERVAL_20261125.md`). The 14-bit
roster curve is fixed at `icv1-fp14-tm43-158b6914`; every `--seed` draws a
different target on the same curve.

The primary degree-32 map (`prime-isogeny:t=5,seed=5`) has 21 abscissae and no
exact `prime-interval:t=5/6` shift match. Auxiliary seed 7 has 15 and is
matched by `prime-interval:t=5,lo=1` (15). The panel therefore tests
`prime-isogeny:t=5,seed=7,aux=ordinary` at 15/15, not seed 5.

Target seeds `20261301`, `20261302`, `20261303`; 16 repeats; m=2 Semaev F4
chain mode, degree 8; 1,000 target trials; five-second solver budget; one rho
context run; mirrored interval/candidate/candidate/interval order.

The first 240-second run (`degree32_14bit_single_curve_20261301.jsonl`) censored
both interval arms (exit 124) while both candidate arms completed. Per
`AMENDMENT_F4_DEGREE32_14BIT_CAP900_20261125.md`, the whole-call cap was raised
to 900 seconds and all three seeds were rerun.

## Results

Ratios are candidate / interval. Lower is better.

| Target seed | Combined GAE | Pair 1 GAE / wall / CPU | Pair 2 GAE / wall / CPU | Relations (int/cand) | Target trials (int/cand) | Median s/rho (int/cand) |
|---:|---:|:--|:--|---:|---:|---:|
| 20261301 | **0.3152** | 0.3070 / 0.3111 / 0.4667 | 0.3245 / 0.3322 / 0.3859 | 360 / 376 | 14,044 / 14,290 | 1,176,907 / 322,761 |
| 20261302 | **0.3550** | 0.3509 / 0.3488 / 0.4344 | 0.3593 / 0.3602 / 0.4458 | 322 / 370 | 11,268 / 12,580 | 671,858 / 223,918 |
| 20261303 | **0.3088** | 0.2940 / 0.2938 / 0.3559 | 0.3254 / 0.3301 / 0.3505 | 354 / 364 | 14,926 / 14,036 | 1,256,379 / 343,349 |
| **Combined** | **0.3241** | all pairs < 0.36 GAE, < 0.36 wall, < 0.47 CPU | | **1,036 / 1,110** | **40,238 / 40,906** | |

Combined charged GAE: 25,598,378,635 interval / 8,296,675,176 candidate. The
component gate passed on all three target replicates; all 48 scalar checks
agreed across the four runs.

Stores:

- `degree32_14bit_single_curve_20261301.jsonl` (240 s, censored baseline)
- `degree32_14bit_single_curve_20261301_cap900.jsonl`
- `degree32_14bit_single_curve_20261302_cap900.jsonl`
- `degree32_14bit_single_curve_20261303_cap900.jsonl`

## Claim boundary

This is a 14-bit component signal for the degree-32 construction at auxiliary
seed 7, on **one curve** with three target replicates. It is not a three-curve
replication, and it is not the primary seed-5 map at 14 bits.

It does not establish a same-target rho crossover. Candidate median
`s_over_rho` remains **2.2e5–3.4e5**. It does not establish a complete cold
pipeline at a larger size, a random-instance or curve-family replication,
asymptotic improvement, or transfer to s59, P-192 or NIST.

The 240-second censoring of the interval arms is itself a scaling signal: at
14 bits the interval arm needs >240 s while the candidate finishes in 64–98 s.
It is recorded as censoring, not as a measured ratio.
