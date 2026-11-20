# H4 n=35 sparse target relation check

A separately identified n35 IC candidate reconstructs the verified target relation over the occupied signed factor-base columns. On the ten exact frozen public points, both candidates returned the same first witness, signed row, and verified scalar in every pair. The median paired dense/sparse target-online ratio was **23.83** (range 11.34–36.27; geometric mean 22.05) on this ordinary local host.

| Pair | Workload ID | Order | Dense online (ms) | Sparse online (ms) | Dense/sparse | Calibration |
| ---: | --- | --- | ---: | ---: | ---: | --- |
| 0 | `We9ffc337e24a` | dense→sparse | 14.978 | 0.807 | 18.55 | pass |
| 1 | `We4ffb7245308` | sparse→dense | 15.640 | 1.379 | 11.34 | pass |
| 2 | `Wcf8608900acd` | dense→sparse | 14.272 | 0.544 | 26.23 | pass |
| 3 | `W8da2b3514fa5` | sparse→dense | 15.152 | 1.055 | 14.37 | pass |
| 4 | `W1f30c904dda9` | dense→sparse | 14.146 | 0.398 | 35.59 | pass |
| 5 | `W206aa6113415` | sparse→dense | 14.958 | 0.614 | 24.38 | pass |
| 6 | `W2329a5f50fdd` | dense→sparse | 14.797 | 0.408 | 36.27 | pass |
| 7 | `W504780d8ac1b` | sparse→dense | 14.749 | 1.002 | 14.71 | pass |
| 8 | `Wa6770de4f197` | dense→sparse | 14.405 | 0.619 | 23.28 | pass |
| 9 | `W24bf5155da6f` | sparse→dense | 14.994 | 0.471 | 31.82 | pass |

| Measured local median | Dense candidate | Sparse candidate |
| --- | ---: | ---: |
| Target-online interval | 14.877 ms | 0.616 ms |
| Target relation check | 14.242 ms | 0.142 ms |

The dense candidate is `IC1N35Ch4randfb4096PDP3mitmRCsampleLAgaussTDdescentISO0hf491f3bcd7b6` and the sparse candidate is `IC1N35Ch4randfb4096PDP3mitmRCsampleLAgaussTDdescentISO0he546b5410907`. The latter has a separate immutable [candidate manifest](candidate.json) because its target relation check and implementation source differ. Both retain the same exact 4,096-point base, 8,321,091-key pair index, 2,048 representative logs, target law, witness selection, row construction, descent, and scalar replay. The sparse check verifies the original three-point group sum and the signed representative sum over at most three occupied columns; the dense check reconstructs over all 2,048 columns.

The measured shared setup to rebuild the base and index and replay the previously solved logs was 150.347 s, excluded from each target-online interval. Peak process RSS was 1,592,475,648 bytes. All ten paired scalar certificates, same-witness checks, and 1.25 calibration-ratio gates passed. The [protocol](protocol.json), [source](sparse.py), [manifest generator](make_manifest.py), [candidate manifest](candidate.json), and [runner](run.py) were committed before measurement in `4782cd6e662`. [result.json](result.json) preserves every arm, phase interval, target, order, and calibration receipt; [verify.py](verify.py) independently rebuilds the curve and first-pair index and replays every witness and scalar. Its [validation](validation.json) records the descriptive statistics.

This is a secondary local implementation comparison with reusable factor logs ready. It measures a target-stage gain on frozen points, while the [primary complete n35 run](../n35_full_pipeline_20261120/README.md) retains the cold preparation and same-point rho reference. A calibrated Tier 0 or Tier 1 panel pairing the sparse candidate with same-point rho and complete phase receipts is the next performance gate. The [proof map](formal/PROOF_MAP.md) records the unchanged structural Lean dependencies and the exact finite-instance bridge checked by the independent replay.
