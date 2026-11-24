# Results: cheaper linear algebra for Diem 2LP IC on prime-order plane quartics

Every number here is **[pcc:unverified]**. The run is on one shared host with planted targets. There is no replay certificate or validator report yet.

## Binding

- Protocol: PROTOCOL.md sha256 `6d07d31639f7…`, frozen in 612f8dee20e before any grid-prime run. It includes the pre-freeze amendment from development data (section 4).
- Binary: `genus_ladder.qla`, sha256 `adddfbfa…d6d1f6`, built from 4faca82a6f0. Source hashes are in `source.sha256`.
- Data: `raw/run.jsonl` and `raw/run131101.jsonl`, hashed in `data.sha256`.
- Analysis: `analysis.json` (sha256 `aa8fade57ee2…`) from the frozen `analyze.py`.
- Bench record: `benchmarks/records/20261124T0719Z-quartic-diem-la.json`.
- Runs: 2026-11-24, 07:19 to 07:39 UTC, at most 2 threads.

## Integrity (all pass)

- 80 runs: 40 per arm, all paired. Every run certified, and `recovered == t_secret` in every row.
- Both arms recover the same logarithm.
- C0: all 40 `2lp-sfb` rows are identical to the parent ladder's rows on the 20 compared fields.

## Decision (PROTOCOL.md section 6): **IMPROVEMENT**

This is a component speed-up of the index-calculus baseline, not a claim against rho.

Paired GAE ratio sge/sfb, pooled with equal weight per prime, cluster bootstrap 95%:

| w | pooled gm | 95% |
|---|---|---|
| 1 | 0.371 | [0.340, 0.406] |
| 0.1 | 0.372 | [0.340, 0.407] |
| 0 | 1.0028 | [1.0010, 1.0047] (tolerance ≤ 1.01) |

The ratio at w = 0 is above 1 only because of the 8 fallback runs. Each has 61–77 more group operations than its sfb pair, about one failed `[t]G = T` check of ~log₂ r operations. The effect is 0.9% at 16411.

Per prime, at w = 1:

| p | 4099 | 16411 | 65537 | 131101 |
|---|---|---|---|---|
| GAE ratio | 0.376 [0.370, 0.383] | 0.431 [0.333, 0.558] | 0.327 [0.300, 0.384] | 0.359 [0.295, 0.432] |
| `la` total ratio | 0.204 | 0.307 | 0.198 | 0.253 |

- Pooled `la_add` ratio: 0.236 [0.205, 0.270].
- Slope in ln p of the w = 1 ratio: −0.037 [−0.090, 0.014]. No trend is resolved.

## IC / ideal negation rho (GAE_w; geometric mean over 5 curves, boot95)

| p | sfb w=0.1 | sge w=0.1 | sfb w=1 | sge w=1 |
|---|---|---|---|---|
| 4099 | 2.38 | 0.896 [0.868, 0.926] | 23.8 | 8.94 |
| 16411 | 1.76 | 0.760 [0.485, 1.19] | 17.6 | 7.59 [4.86, 11.9] |
| 65537 | 0.896 | 0.294 [0.251, 0.386] | 8.96 | 2.93 [2.51, 3.86] |
| 131101 | 0.775 | 0.278 [0.202, 0.382] | 7.75 [6.75, 8.85] | **2.78 [2.02, 3.80]** |

- At w = 0, IC/rho is about 2e-4 at 131101 for both arms.
- Every prime is **UNRESOLVED** for both arms: IC/rho is below 1 at w = 0 and above 1 at w = 1.
- **Distance remaining at 131101, w = 1: 2.78×**, down from 7.75×.

## Where the remaining cost is (`la_breakdown`; descriptive, outside the decision rule)

- Linear algebra is now 42–77% of the sge arm's native operations, against 78–89% for sfb. Sparse products are still 85–96% of what remains.
- In the sge arm, runs with an `la_fallback` carry 83%, 46% and 77% of the linear algebra at 16411, 65537 and 131101. Their pair ratio is about 0.66.
- Pairs that certify in one attempt have `la` ratios of 0.204, 0.184, 0.173 and 0.168 at the four primes.
- Every fallback, in development and in the study, falls on a (curve, seed) where `2lp-sfb` also needed another attempt. In those runs the fully counted fallback re-runs a whole-core solve that also fails.
- Next lead, needing a new protocol: decide failure from the square subsystem, or reuse the solve across attempts, then skip the fallback. Doing so is not yet shown to be sound.

## Figure

`fig_la.svg` and `fig_la.png`. The PDF report is `reports/run-20261124T0719Z/report.pdf`.
