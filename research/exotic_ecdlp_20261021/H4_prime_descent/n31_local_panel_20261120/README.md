# H4 n=31 five-target local panel

The frozen n=31 candidate and installed Sage 20-partition rho each recovered and independently replayed the scalar for all five distinct, precommitted subgroup points. Arm order alternated IC/rho and rho/IC. The median paired target-online ratio was 57.26 (rho/IC) across these five points on one ordinary local macOS host. Every pre/post 512-addition calibration ratio passed the frozen 1.25 gate. These are exploratory local timings; Tier 0 or Tier 1 host evidence is needed for a promoted CPU wall-time comparison.

| Target | Arm order | IC online ms | Same-point rho online ms | Rho / IC | Calibration ratio |
| ---: | --- | ---: | ---: | ---: | ---: |
| 0 | IC, rho | 7.024 | 489.847 | 69.74 | 1.018 |
| 1 | rho, IC | 7.762 | 478.615 | 61.66 | 1.027 |
| 2 | IC, rho | 6.810 | 136.022 | 19.97 | 1.011 |
| 3 | rho, IC | 7.866 | 450.396 | 57.26 | 1.002 |
| 4 | IC, rho | 7.412 | 230.923 | 31.15 | 1.044 |

The single shared index reconstruction and replay of all 1,045 factor logs took 37.425 s outside the five target clocks. Each target's IC interval includes the five exclusive online phases and scalar replay; each rho interval includes scalar replay on the same original point. All five IC arms used a direct three-point witness. The full candidate's empty-cache preparation receipt remains in the `n31_full_pipeline_20261120` packet. The curves, subgroups, cofactors, and bases differ between n29 and n31, so this panel does not isolate field-degree scaling.

The [protocol](protocol.json), [runner](run.py), and exact [fixture points](fixtures.json) were committed before measurement in `9a3ce2d73e4`. The checked Sage runtime is in [sage-runtime-info.json](sage-runtime-info.json), and all arm outcomes and timings are in [result.json](result.json). [verify.py](verify.py) independently checks fixture exclusions, point identities, each witness and signed-log equation, scalar certificates, first-hit scans, phase sums, calibration records, and the paired arm order. Its [validation receipt](validation.json) passed; all five [IC v2 one-target rows](runs.jsonl) passed the measurement contract, recorded in [contract-validation.txt](contract-validation.txt).

The next comparison should repeat the same frozen paired-target design on a dedicated Tier 1 instance or isolated Tier 0 host, retaining all censored and noisy pairs. The structural Lean proofs for pair lookup, cofactor projection, and shifted-target recovery are referenced in the complete candidate packet; this panel adds no new mathematical theorem.
