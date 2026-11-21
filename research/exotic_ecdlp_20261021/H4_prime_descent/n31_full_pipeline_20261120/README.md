# H4 n=31 complete one-target pipeline

The selected n=31 H4 curve supports a complete one-target group-index pipeline. Its 2,090-point cofactor-cleared base folds to 1,045 signed columns. All 2,737 ordinary relation queries returned verified three-point decompositions; 1,044 raised rank beyond the generator anchor. The 2,738-row matrix reached full rank and solved all 1,045 representative logarithms, each independently replayed as an elliptic-curve point multiple. On a fresh public point `(1818443913, 706820564)`, the pipeline recovered scalar 27,775,558, independently replayed it, and installed Sage rho recovered and replayed the same scalar on that point.

The field is GF(2^31) in the exact polynomial representation recorded by [candidate.json](candidate.json). The selected curve has order 2,147,394,118 = 62 × 34,635,389. The subgroup generator, full base, candidate ID, workload ID, and target are in the machine-readable records. The target was made from a SHA256-derived scalar solely to construct a public-point fixture; that scalar was excluded from all relation queries. Both online intervals begin with a ready factor-log and pair index for IC, or at rho's first target-dependent walk step, and end after scalar replay. This run used one ordinary local macOS host and one worker per arm.

| Measured stage or result | Value |
| --- | ---: |
| Actual subgroup base / signed columns | 2,090 / 1,045 |
| Ordered pair attempts / distinct stored sums | 4,368,100 / 2,116,611 |
| Ordinary queries: verified / exhausted | 2,737 / 0 |
| Relation rows / final rank | 2,738 / 1,045 |
| Cold preparation, empty cache | 39,591.593 ms |
| Factor-base construction / pair-index build | 3,587.111 / 33,422.979 ms |
| Relation PDP / final relation LA | 512.442 / 1,540.893 ms |
| IC target-online interval | 7.410 ms |
| Same-point rho target-online interval | 148.829 ms |
| Paired online ratio, rho / IC | 20.09 |
| Peak observed RSS | 1,556,496,384 bytes |

The IC target needed one direct lookup with four third-point attempts; no shift was used. The five exclusive target-online phases and eleven cold phases are in [result.json](result.json). The ratio is a single exploratory local observation. A Tier 0 or Tier 1 paired panel with alternating order and calibration is needed for a promoted CPU wall-time comparison. The n29 and n31 curves, subgroup orders, cofactors, and bases differ, so the two packets do not isolate a field-degree scaling effect.

## Freeze, replay, and proof map

The [protocol](protocol.json) and [runner](run.py) were committed before execution in `45b9c7fe365`. The [checked Sage runtime](sage-runtime-info.json) was recorded before the measured run. The runner retains every query status and rank change, all factor logs, phase intervals, target witness, and rho scalar. The result JSON is compacted after execution without changing its parsed content, keeping the complete trace in Git below the artifact-migration threshold. [verify.py](verify.py) independently rebuilds the curve and base, constructs a point-only pair-sum set, checks each query and rank change, solves the matrix again with Sage, replays every factor log, and checks the target and rho certificates. Its [validation receipt](validation.json) passes, and the [v2 IC measurement row](run.jsonl) passes the repository IC measurement contract v2, with output in [contract-validation.txt](contract-validation.txt).

The algebraic pair-lookup completeness theorem is `H4PairIndex.pair_lookup_iff` in the existing n=19 coverage packet. `H4CofactorProjection.additive_map_preserves_three_sum` proves transport under the clearing map, and `ShiftedTargetRecovery` in the n29 pipeline packet proves the general shifted-target log identity. Those Lean modules were checked with their stated assumptions and `#print axioms`; the n31 finite-instance counts and timing are audited computations, not Lean cardinality theorems.

The next measurement is a precommitted alternating-order same-target n31 IC/rho panel on distinct targets under a calibrated Tier 1 or isolated Tier 0 allocation. Until that environment exists, this packet is the complete local correctness and stage-cost result.
