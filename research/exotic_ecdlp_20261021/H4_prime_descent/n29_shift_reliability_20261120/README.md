# N29 shifted-target recovery reliability

The frozen n29 candidate recovered and independently replayed the scalar for **all 256 new public subgroup targets**. Direct lookup succeeded for **138**; the other **118** targets were recovered after one to six known-scalar shifts. No target reached the frozen 16-scan cap. The independent Sage audit rebuilt the 573,927-element pair-sum set, replayed every exhausted scan and successful first hit, and checked all 256 original-point scalar certificates.

| Shifted-point lookups per target | Targets |
| ---: | ---: |
| 1 (direct hit) | 138 |
| 2 | 65 |
| 3 | 30 |
| 4 | 15 |
| 5 | 6 |
| 6 | 1 |
| 7 | 1 |
| 8–16 or cap reached | 0 |

The 256 targets required **461 total scans**: 256 successful scans and 205 exhausted scans. The median target-online interval, including every failed shift and final scalar replay, was **9.460 ms**; the mean was **14.829 ms**, and the maximum was **70.088 ms**. Rebuilding the immutable base and first-pair index and replaying all 536 logs once took **10.843 s** outside the target clocks. The process peaked at 588,447,744 bytes RSS. A 512-addition calibration before and after the panel had ratio 1.039. These are ordinary-host component timings; the earlier five same-point IC/rho pairs remain the primary one-target comparison.

[`fixtures.json`](fixtures.json) fixes the exact 256 public points and workload IDs. The generator excludes all prior n29 relation-query scalars and target samples, with hashes pinned in [`protocol.json`](protocol.json). The fixture, protocol, and runner were committed in `d4d51da77ca` before measurement. [`result.json`](result.json) retains all 256 statuses, shift scalars, failed-scan counts, witnesses, exclusive online phases, and correctness certificates. [`sage-runtime-info.json`](sage-runtime-info.json) records the checked runtime. [`audit.py`](audit.py) independently reconstructs the base and every target path; its [`validation.json`](validation.json) passes.

The **138/256 direct-hit fraction (53.906%)** is an empirical result for this frozen deterministic target set. The previous disjoint 256-target coverage diagnostic also had 138/256 direct hits. The shift-count sequence is consistent with a geometric model with success probability near that direct fraction; this is a model comparison, not an independence theorem. The finite panel establishes zero observed cap hits under its exact target law and resource cap. The structural subtraction of each known shift is covered by the checked `H4ShiftedTargetRecovery.recover_from_shift` theorem in the complete-pipeline packet.

Reproduce the audit from the repository root:

```sh
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/H4_prime_descent/n29_shift_reliability_20261120/audit.py
```
