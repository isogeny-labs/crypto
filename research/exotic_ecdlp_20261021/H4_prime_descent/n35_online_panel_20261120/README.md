# H4 n=35 fresh-point paired online panel

Ten frozen fresh public points were each solved and independently scalar-verified by the n35 packed-index IC candidate and installed Sage rho. The order alternated IC/rho and rho/IC. All ten before/after point-addition calibration checks passed the frozen 1.25 ratio gate. The median paired rho/IC online ratio was **112.79**, with descriptive range 35.59–285.72 and geometric mean 108.85.

| Pair | Workload ID | Order | IC online (ms) | Same-point rho online (ms) | Rho/IC | Calibration |
| ---: | --- | --- | ---: | ---: | ---: | --- |
| 0 | `We9ffc337e24a` | ic→rho | 14.710 | 2205.613 | 149.94 | pass |
| 1 | `We4ffb7245308` | rho→ic | 14.705 | 2640.950 | 179.60 | pass |
| 2 | `Wcf8608900acd` | ic→rho | 13.375 | 826.230 | 61.77 | pass |
| 3 | `W8da2b3514fa5` | rho→ic | 15.124 | 3060.821 | 202.38 | pass |
| 4 | `W1f30c904dda9` | ic→rho | 14.283 | 854.269 | 59.81 | pass |
| 5 | `W206aa6113415` | rho→ic | 14.152 | 3760.099 | 265.69 | pass |
| 6 | `W2329a5f50fdd` | ic→rho | 14.558 | 4159.514 | 285.72 | pass |
| 7 | `W504780d8ac1b` | rho→ic | 14.634 | 830.353 | 56.74 | pass |
| 8 | `Wa6770de4f197` | ic→rho | 14.247 | 1077.535 | 75.63 | pass |
| 9 | `W24bf5155da6f` | rho→ic | 14.084 | 501.277 | 35.59 | pass |

The median IC interval was 14.421 ms (range 13.375–15.124 ms); median rho was 1641.574 ms (range 501.277–4159.514 ms). Each IC target used its first, unshifted decomposition. Shared setup to rebuild the 4,096-point base and 8,321,091-key index and replay all 2,048 factor logs took 144.866 s; it is recorded once and excluded from each target-online interval. Peak process RSS was 1,531,101,184 bytes.

This is a secondary ten-target exploratory panel on an ordinary local macOS host. The [one-target full run](../n35_full_pipeline_20261120/README.md) remains the primary candidate result. Per-point scalar certificates, both arm intervals, calibration times, order, and resource observations are in [result.json](result.json). The [fixtures](fixtures.json), [protocol](protocol.json), and [runner](run.py) were committed before measurement in `b4caa61b621`. [verify.py](verify.py) rebuilds the curve, base, and pair table independently, checks each frozen public point and first witness, and replays both scalar certificates. Its [summary](summary.json) records the panel statistics after verification. The structural Lean proofs and exact assumptions remain mapped in the [full candidate proof map](../n35_full_pipeline_20261120/formal/PROOF_MAP.md). A Tier 0 or Tier 1 same-point alternating-order panel with exposed topology and noise receipts is the next performance gate.
