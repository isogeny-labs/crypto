# L1 paired protocol: result

Frozen protocol: `../PROTOCOL.md`, `../protocol.json`. Freeze commit
`5542605cf81332eaaa9df16e16e7a053e4343867` (2026-11-19T16:54:40Z), protocol
SHA-256 `1766ee66…436d2d`. `run.sh` ran unmodified at the freeze commit, and
every line of `SOURCES.sha256` verified. All results are [pcc:unverified].
The run was local, on one thread, with no paid compute.

## Verdict: INCONCLUSIVE (the interval straddles the 1.5 line)

| Quantity | Value |
|---|---|
| Fresh targets | 96 measured of 112 generated. All unseen, 0 refused, 0 collection failures. |
| Residuals | 393,216 primary and 98,304 A/A. |
| F, replay-verified full splits | 555 |
| M, replay-verified L1 rows | 789: (1^4,2) 650, (1^2,2^2) 134, (2^3) 5 |
| ρ̂ = M/F | 1.4216 |
| Clopper–Pearson 95% | 1.2737–1.5875. Recomputed independently by the orchestrator: identical. |
| Cluster bootstrap 95% | 1.2694–1.5893 (B = 20,000) |
| Arm B / arm A success rate | 2.42x |
| Measured class match rate f̂ | 0.0776. The model's ρ at f̂ is 1.441. |
| CPU | 2.90 CPU-h of an 8 h budget. Not truncated. |
| drand | Round 6655996 from api.drand.sh. The fallback was not used. |

The lower bound is below 1.5, so it is not PROMOTE. The upper bound is above 1.5, so it is not KILL.

**What is established.** On fresh targets, accepting L1 rows raises the number of verified descent successes per solution by a factor of 2.42 (1 + ρ̂) on the same streams, which is the acceptance gain the model predicts. The registered 1.5 line, which the step-2/3 README noted sits on the model's own value, is neither cleared nor refused. No descent-cost or wall-time saving is claimed (critic F6).

## Controls, all passed
- **PC1.** Exact reproduction of Run50 and of steps 2–3 on the new instrument, 27 of 27 checks.
- **PC2.** Two planted targets each return the owner's answer.
- **Negative control.** 984 of 984 sign-flipped L1 rows are refused.
- **A/A.** On 24 targets, all three 99% intervals contain 1.
- **Determinism.** Target 0000 is byte-identical across two runs.
- **Freshness.** Every candidate passed the generator's check and the unseen check.
- **Row replay.** For every L1 row, 0 failures across group identity, anchor witness, affine path, kernel projection and elliptic replay.

## Deviations, per the runner
1. The nohup log is beside `$S`, not inside it, because `run.sh` requires an empty `$S`.
2. The cargo target directory is the protocol's own fresh one.
3. The runner wrote no README; the orchestrator wrote this one. research-kb was unavailable, and no novelty claim is made.

## Files
- Here: `analysis.json`, `targets.tsv`, `aa.tsv`, `controls.kv`, `pc1.json`, `refused.tsv`, `publics.jsonl`, `nonce.txt`, `freeze.txt`, `cpu.tsv`, `build.log`, `RAW.sha256.part00..02`, `COMMIT.sha256`.
- Raw outputs, 312 MB, are LOCAL in the runner's scratch directory. Their per-file SHA-256 values are in the `RAW.sha256` parts.
