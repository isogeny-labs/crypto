# Cross-curve validation of the combined solver

All twelve preselected field/genus pairs produced eligible curves. All four arms recovered and verified all 48 scalars. No input was substituted or censored. The genus-dependent policies were frozen before instance selection. Each speedup below is the original normalized solver median divided by the combined solver median on the identical input.

| Genus | New curves | Verified targets | Full median speedup range | Targets at tenfold |
|---|---:|---:|---:|---:|
| 2 | 4 | 16 | 2.452–6.844× | 0 |
| 3 | 4 | 16 | 2.843–4.339× | 0 |
| 4 | 4 | 16 | 4.182–13.376× | 1 |

Genus four p=37, c=9, ell=520447, case 2 reached 13.376× (1.966250 ms versus 0.147000 ms). The original needed 210 anchors and 239,938 reductions, the combined solver 72 and 32,622 respectively. On this curve the scan already covered all 20 catalogue entries in arm 1; arm 3 could not expand it. Arm 1 independently recorded 0.154834 ms on the same input. Credit the existing full algorithm for this crossing. In genus three the added policies intentionally do nothing, so inter-arm timings do not measure a further algorithmic gain there. All twelve curves belong to the previously used polynomial families, and this run does not test unrelated families or establish an asymptotic exponent.

The fixed combined policy exceeded tenfold on a new field and target, while the other 47 targets stayed below tenfold. Do not pool these single-host medians with the development panel. RUN.txt identifies the prospective commit. CONTROL.txt records the chosen coefficients through c and family, subgroup orders, 192 counted executions and 2,304 rotating timing observations. summarize.rb reconstructs COMPARISON.tsv.
