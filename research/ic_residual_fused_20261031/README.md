> **Pricing note**: the w > 0 figures here (fused/baseline 0.91-1.30) price the baseline's table bytes as operations (see `../ic_residual_dp_eq_20261101/README.md`); w = 0 is unaffected.

# Fused residual step (targets=residual-fused): result

Protocol: [PROTOCOL.md](PROTOCOL.md) (frozen before data; Amendment 1: local L0 run, counted units only).
Reproduce: `scripts/make_specs.sh; scripts/run_all.sh; scripts/analyse.sh; python3 scripts/summarise.py`.
All four sessions: 512/512 records verified, `ecbench verify --replay 12` audit OK (analysis/verify-*.txt).
A/A (res4 vs res4aa) ratio 1.0000 on every curve (deterministic counted units).

## Result (paired by instance, 16 targets x 3 rounds per curve; fused / baseline, counted S)
| curve | B | w=0 [95% CI] | w=0.1 | w=1 |
|---|---|---|---|---|
| p16 | 2 / 4 / 8 | 0.840 [0.788,0.896] / 0.874 [0.838,0.911] / 0.917 [0.883,0.952] | 0.948 / 1.026 / 1.099 | 1.061 / 1.132 / 1.176 |
| p18 | 2 / 4 / 8 | 0.769 [0.720,0.823] / 0.762 [0.728,0.796] / 0.805 [0.771,0.840] | 0.946 / 0.970 / 1.068 | 1.090 / 1.081 / 1.155 |
| p36 | 2 / 4 / 8 | 0.568 [0.493,0.655] / 0.662 [0.611,0.718] / 0.618 [0.576,0.663] | 0.925 / 1.133 / 1.113 | 1.103 / 1.297 / 1.219 |
| p37 | 2 / 4 / 8 | 0.561 [0.495,0.635] / 0.612 [0.560,0.669] / 0.595 [0.554,0.640] | 0.912 / 1.048 / 1.072 | 1.088 / 1.199 / 1.175 |

Distance to matched rho (rho -> arm, S ratio; w=0 / w=1): res4 p36 3.61 / 46.9, res4f p36 2.39 / 60.8; res4 p37 4.10 / 53.3, res4f p37 2.51 / 63.9.

## Reading (decision rule as frozen)
- Mechanism verified: gae/step falls from 2.005 to 1.004 (p36).
- C1 as preregistered (interval below 1 at w = 0, 0.1 AND 1) is **not met**: the improvement holds at w = 0 on all 12
  (curve, B) cells (CI below 1), is mixed at w = 0.1 and reversed at w = 1. The fused walk needs 1.1-1.3x more steps
  per harvested relation (p36: 455k vs 401k steps at B=2, 580k vs 438k at B=4), and the per-step table work is unchanged,
  so priced native work eats the saving. Per the section 5a rule the order is "unresolved inside the band".
- Classification: a w = 0 stage/engineering improvement of the group-operation count of the residual collector
  (generic collision search, not structural IC); not a pipeline improvement across the band. Recorded as such.
- Next: find out why the fused walk needs more steps (hash-bit correlation of j and i is one hypothesis, untested here).
