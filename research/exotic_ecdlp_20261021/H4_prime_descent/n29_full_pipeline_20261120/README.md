# H4 n29 complete one-target pipeline

The frozen `n=29` instance-1 candidate collected enough ordinary relations to solve all **536 signed factor-base logarithms**, independently replayed every log as a subgroup point, and recovered a fresh public target scalar **188,743,542**. Its first direct target scan exhausted the 1,072-point base; one known-scalar shift yielded a verified triple, from which the candidate recovered the original scalar. Installed Sage's 20-partition rho independently recovered and replayed the same scalar on the same public point.

| One-target exploratory record | IC | Same-point Sage rho |
| --- | ---: | ---: |
| Public target | `(132274585,182371300)` | `(132274585,182371300)` |
| Verified scalar modulo 268,447,807 | 188,743,542 | 188,743,542 |
| Online wall time including scalar replay | 28.589 ms | 1,851.050 ms |
| Observed online ratio `rho / IC` | **64.75** | — |
| Target-independent IC preparation | 29,536.999 ms | — |
| IC preparation plus one target | 29,565.588 ms | — |

This is a complete correctness and phase-cost result on one ordinary macOS host. The online ratio is exploratory; there is no Tier 0 or Tier 1 isolation receipt or alternating multi-target panel for a promoted speedup. The single-target online interval starts when target-dependent computation begins with the reusable pair index and factor-base logs ready. The cold preparation is recorded separately. The known fixture scalar was used to construct the public point and for independent replay, outside the online interval; it was excluded from relation-query scalars.

The immutable candidate is `IC1N29Ch4randfb1072PDP3mitmRCsampleLAgaussTDdescentISO0hb60653be7f42`, curve `EC1N29Ch4randh1775caf67a64`, workload `W48d40342da04`. [`candidate.json`](candidate.json) fixes the exact 1,072-point base, 536 inverse-folded columns, first-pair index, known-scalar relation stream, modular Gaussian rank solver, and 16-shift target policy. [`workload.json`](workload.json) fixes the one public target and construction law. The protocol and runner were committed before execution in `b163bb3fef5`.

## Charged work

The candidate attempted **2,575 ordinary relation queries**. Of these, **1,405** produced group-verified triples and **1,170** exhausted a complete base scan; 535 verified rows raised rank beyond the generator anchor. The final relation matrix has 1,406 rows, 536 columns, and rank 536. The independent verifier rebuilt every row and rank gain, solved the matrix again, and replayed every representative log.

| Cold stage | Time |
| --- | ---: |
| Curve/setup and control overhead | 252.351 ms |
| Factor-base reconstruction and subgroup checks | 2,041.352 ms |
| First-pair index, 1,149,184 attempted pairs / 573,927 stored sums | 9,672.609 ms |
| Relation-query point generation | 201.060 ms |
| All relation lookups, including 1,170 exhausted scans | 16,830.227 ms |
| Group verification / row construction | 40.389 / 22.141 ms |
| Incremental rank and final linear solve | 456.370 ms |
| Replay of all 536 solved logs | 20.500 ms |
| **Cold preparation total** | **29,536.999 ms** |

The target's two shift attempts were charged to its 28.589 ms online interval. Shift `t=0` exhausted 1,072 third-point attempts. Shift `t=56,907,240` found a verified triple after 346 third-point attempts. The five exclusive target phases are: query/shift construction 0.117 ms, pair lookups 23.979 ms, relation checks 4.393 ms, scalar descent 0.031 ms, and final original-point replay 0.069 ms. Peak RSS was 546,914,304 bytes. The checked Sage runtime is in [`sage-runtime-info.json`](sage-runtime-info.json).

## Independent verification and proof map

[`verify.py`](verify.py) reconstructs the curve and exact base, builds a point-only pair-sum set independently of the candidate's first-pair table, checks all 2,575 outcomes including exhausted scans, recomputes the full rank trajectory, solves the 1,406-row matrix with Sage, and replays all 536 logs. It then checks the target's shift sequence, first successful scan position, signed row equation, subtraction of the known shift, original-point scalar replay, and rho replay. The [validation receipt](validation.json) passes. The [v2 measurement row](run.jsonl) passes the repository's IC contract; output is in [`contract-validation.txt`](contract-validation.txt). The [result](result.json) retains every query and phase measurement.

The algebraic shift step is Lean-checked in [`ShiftedTargetRecovery.lean`](formal/ShiftedTargetRecovery.lean): from `Q + t • G = s • G`, it proves `Q = (s-t) • G` for any module over a ring. The [proof map](formal/PROOF_MAP.md) records its hypotheses and the remaining concrete curve/matrix bridge. The earlier Lean pair-lookup and cofactor-transport theorems apply to the structural steps; the finite-instance curve, matrix, and scalar certificates are independently replayed computations.

Reproduce from the repository root with the checked Sage launcher:

```sh
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/H4_prime_descent/n29_full_pipeline_20261120/verify.py
```

The next comparison is five distinct precommitted targets with the same frozen candidate and Sage rho in alternating order, then a Tier 1 isolated panel under the repository's CPU benchmark contract.
