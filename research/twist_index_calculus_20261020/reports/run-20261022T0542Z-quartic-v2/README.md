# Quartic twist24 full-cost v2 run

UTC start 2026-10-22 05:42:50.656 (output file birth); completion 05:43:39.009. Shared Intel Xeon 6975P-C x86-64 host, 32 logical CPUs, `rustc 1.99.1`, `RAYON_NUM_THREADS=16`, L0. No isolated wall-time claim.

Frozen [protocol](../../QUARTIC_TWIST_LINE_PROTOCOL.md). Producer source pinned at commit `4d6cb3168e`, `quartic_folds.rs` SHA-256 `f23ce44e224b0fb9fa49bd5e9308f748b784155c65b1db30278856e6e7aa142e`. Exact command:

```sh
RAYON_NUM_THREADS=16 TMPDIR=/home/ec2-user/.cache/gls-report-tmp timeout 1800s target/release/examples/quartic_twist24 > research/twist_index_calculus_20261020/quartic_twist24_full_v2.ndjson 2> research/twist_index_calculus_20261020/quartic_twist24_full_v2.log
```

The run completed all 21 registered curve/seed instances: 63 flat/cubic/twist24 IC solves, 1,008 matched rho solves, zero failures, and 1,821 selected relations. Every scalar was `271` and independently replayed by the [native audit](../../quartic_twist24_native_audit_v2.json). Raw [NDJSON](../../quartic_twist24_full_v2.ndjson) SHA-256 `2c050172a0136bf38185ca54ec1a558346548b10bb9b8463a8b77497c0e6aa34`; [stderr](../../quartic_twist24_full_v2.log). The [native/plot summary](../../quartic_twist24_summary_v2.csv) preserves all seven curve medians and three-seed ranges; the graph below shows same-base query, row-operation and recorded field-multiplication ratios, plus the field-multiplication gap to full24 rho.

![Quartic twist24 v2 count comparison](quartic_twist24_cost_v2.svg)

The combined fold uses four times fewer log columns than cubic-only on every fixed physical base. Median cubic/twist residual-query ratios range 2.57–3.81 and row-operation ratios 10.78–14.79; recorded field-multiplication ratios range 0.998–3.80. This is a phase/count result. Hash operations, IC-stream group additions and a calibrated complete `S` are not present; wall milliseconds are L0 descriptive. See the [results report](../../QUARTIC_TWIST24_REPORT.md) and [PDF](README.pdf).
