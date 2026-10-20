# Five-lead choice-two operation costs

This browsing copy documents the complete two-choice genus-2 cohort from the
[five-lead study](../../research/five_leads_e2e_20261019/REPORT.md). The frozen
price grid contains 21 curves at seven source-order sizes (16, 20, 24, 28, 32,
36 and 40 bits), with 60 paired one-target workloads per curve (1,260 paired
targets total). All six arms were source-verified and independently replayed:
6,300 records.

The fixed price is `M + 0.5S + 10I` plus `0.1M` per declared native logical
operation. Ratios divide sums of paired workload costs. Each size point pools
180 targets; the pooled reference uses all 1,260. A ratio above 1 means the
two-choice candidate incurred higher operation cost.

| Source order bits | Paired targets | Choice2 / incumbent | Choice2 / source-negation rho |
| ---: | ---: | ---: | ---: |
| 16 | 180 | 0.943697 | 3.812897 |
| 20 | 180 | 0.972590 | 3.824903 |
| 24 | 180 | 0.981876 | 4.229300 |
| 28 | 180 | 0.988278 | 4.373698 |
| 32 | 180 | 1.018553 | 4.366459 |
| 36 | 180 | 1.027152 | 4.694925 |
| 40 | 180 | 0.993748 | 4.444705 |
| **Pooled** | **1,260** | **1.000857** | **4.483247** |

These are L0 operation-count estimates. There is no isolated wall timing,
A/A timing, hardware-speed claim, fastest-external comparison, or established
end-to-end DLP advantage. The study does not enter performance history or the
IC leaderboard.

## Artifacts

- [TeX report source](report.tex), [PDF build note](PDF_BUILD_BLOCKER.md)
- [SVG graph](choice2-operation-cost-ratios.svg), [150 dpi PNG graph](choice2-operation-cost-ratios.png)
- [Renderer source](render.py)
- [Frozen price grid](genus2-choice2-price-grid.csv), [cohort summary](genus2-choice2-summary.json)

The CSV and JSON are copies of the source study's frozen artifacts. To
reproduce the chart and TeX source from the research directory, run:

```sh
python3 research/five_leads_e2e_20261019/reports/choice2-operation-costs-20261019/render.py
```

The renderer reads the frozen CSV only; it computes the fixed-price ratios and
draws the recorded counts. It does not measure wall time or infer a speedup.
