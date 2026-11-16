# N83 fresh 16-point confirmation: 112 paired recoveries admitted

The [exact-head GitHub Actions run](https://github.com/aburan28/crypto/actions/runs/40175406203)
completed both eight-point Tier 1 strata on 16 fresh public targets. The
independent verifier admitted **112 of 112** candidate–target IC/rho rows and
both host strata. Replaying the downloaded raw screen and stratum artifacts
locally produced an [aggregate](results/40175406203/aggregate.json) byte for
byte identical to the hosted aggregate (SHA-256
`38ec0d47be6c767e992e6505977ef9a292c56776427123a64471e37a54b745ee`).

K1115 has the lowest descriptive geometric-mean online time, 47.114 ms, and
K1111 has the lowest descriptive mean cold process time, 159.263 s. The
frozen exact sign-flip rule uses Bonferroni `alpha = 0.05/21 = 0.00238095`;
both metric selectors returned `BOUNDED_UNKNOWN` with `selected_k: null`.
For example, K1115 versus K1111 online has `p = 0.958801`, and K1111
versus K1116 cold has `p = 0.773010`. The next cross-family comparison must
retain the unresolved near-frontier arms or use a newly sealed fresh-point
selection rule; these 16 points cannot be reused as fresh holdout points.

## Recorded measurements

The exact curve is `EC1N83Ckb1a1haacb69ee9f37`: polynomial-basis
`F₂^83`, `y² + xy = x³ + x² + 1`, subgroup order
`8569786107849059`. Each factor base has `166K` distinct subgroup-usable
points and `K` signed-Frobenius folded columns. The 16 targets have separate
one-target workload IDs and were disjoint from the earlier screen; each row
records the candidate ID, workload ID, run ID, target-dependent IC interval,
independent same-point rho interval, cold process interval, stage costs,
resource receipts, and scalar replay.

| K | Actual base points | Panel online geometric mean, ms | Panel cold mean, s | Median paired rho/IC online ratio |
| ---: | ---: | ---: | ---: | ---: |
| 1103 | 183,098 | 142.358 | 162.320 | 16.632 |
| 1107 | 183,762 | 72.916 | 160.806 | 28.414 |
| 1109 | 184,094 | 99.303 | 165.435 | 14.045 |
| 1110 | 184,260 | 77.205 | 165.329 | 28.296 |
| 1111 | 184,426 | 48.226 | **159.263** | 45.384 |
| 1115 | 185,090 | **47.114** | 165.754 | 27.967 |
| 1116 | 185,256 | 58.977 | 159.395 | 56.961 |

The table's panel summaries are secondary distribution diagnostics. For the
preregistered public point 0, workload `6c74da4d8eb8`, the independently
replayed single-target online intervals and ratios are:

| K | IC online, ms | Same-point rho online, ms | Rho/IC |
| ---: | ---: | ---: | ---: |
| 1103 | 138.865 | 2,269.306 | 16.342 |
| 1107 | 30.838 | 3,060.622 | 99.249 |
| 1109 | 362.887 | 1,454.658 | 4.009 |
| 1110 | 119.916 | 2,435.282 | 20.308 |
| 1111 | 82.283 | 3,611.038 | 43.885 |
| 1115 | 47.840 | 3,714.787 | 77.650 |
| 1116 | 29.096 | 2,324.414 | 79.889 |

The point-0 rho walks use the frozen candidate-specific seeds on the same
public point. Process launch, base/index/log preparation and fixture scalar
construction are outside the target-dependent online intervals; cold process
time is reported separately. The aggregate retains every exclusive online
phase and raw attempt for audit.

## Source, host, and replay

Both strata used commit `f8245a00b19a55b307850a1b9160231533139b7b`,
IC source SHA-256 `58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8`,
and the same IC executable SHA-256
`f94c6cc1ca52252d27bbc1864ca8fa6e4be4f559ae2c099b6486d265a04691c9`.
The rho source and executable hashes are frozen in the
[custody receipt](results/40175406203/receipt.json). Stratum 0 ran on an
AMD EPYC 7763 GitHub Actions runner and stratum 1 on an Intel Xeon Platinum
8573C runner, each with visible affinity `[0, 1]`. Each point's candidates
and rho controls were paired within one stratum. The independent verifier
admitted both Tier 1 host envelopes, their noise/resource gates, the exact
base and rank, every scalar replay, and all 112 timing rows. This is Tier 1
cloud-instance evidence under [the benchmark contract](../../docs/IC_BENCHMARK_TIERS.md).

The [receipt](results/40175406203/receipt.json) records the GitHub artifact
IDs and deterministic SHA-256 roots for the extracted screen and both raw
strata. After downloading those named artifacts from screen run
`40150483792` and confirmation run `40175406203`, replay from the repository
root with:

```sh
PYTHONDONTWRITEBYTECODE=1 python3 -m research.n83_nearby_grid_confirm16_20261024.aggregate \
  /absolute/path/s0/n83-confirm-results-s0 \
  /absolute/path/s1/n83-confirm-results-s1 \
  --screen-output /absolute/path/screen/n83-nearby-grid-results \
  --output /absolute/path/replayed-aggregate.json
```

`cmp` of `replayed-aggregate.json` against the versioned aggregate passed.
The source and raw evidence support the current-source nearby-grid
confirmation; broader K2304–K5500 comparisons remain the next selection
stage.
