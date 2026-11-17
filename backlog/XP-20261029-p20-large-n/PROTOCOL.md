# 20-bit prime-field IC: a larger factor base (N 64) against the N 45 baseline — protocol

Follow-up to [`XP-20261016`](../XP-20261016-prime-base-oracle-tuning/PROTOCOL.md).
That study made `prime-abscissa:size=45` + `mitm:negation_folded=1` the
20-bit prime-field IC baseline (paired ratio 0.818 / 0.798 / 0.726 against
N 32 at w = 0 / 0.1 / 1; second-host replay passed, PR #2821). Its README
records a limit: on the 20-bit train curve N 64 was lower than N 45 at w = 1
(19.46 against 22.10, 8 runs per cell) and higher at w = 0.1 (11.413 against
11.243). N 64 was the largest train size, and the holdout never ran it.
This card tests that one candidate on fresh data.

## 1. Question and class

Does `prime-abscissa:size=64` lower the whole-pipeline counted cost S of a
20-bit prime-field `ic.pipeline` solve against the N 45 baseline, with the
same oracle, across the §5a pricing band? Class: engineering (one parameter,
one size). No exponent claim, no claim at other sizes.

## 2. Arms (`spec-holdout.json`, every arm in one session)

| arm | role | factor base | decision-bearing |
|:--|:--|:--|:--|
| `inc` | baseline | `prime-abscissa:size=45` | yes |
| `n64` | candidate | `prime-abscissa:size=64` | yes (the only candidate) |
| `n90` | control | `prime-abscissa:size=90` | no; descriptive slope beyond 64 |
| `n32` | control | `prime-abscissa:size=32` (the pre-XP-20261016 baseline) | no; consistency check of the earlier result |
| `inc-aa` | control | = `inc` | A/A |
| `rho-neg` | reference | `rho.negation` | matched rho |
| `rho-neg-aa` | control | = `rho-neg` | A/A |

Every IC arm: `oracle = mitm:negation_folded=1`, `max_trials = 1410236`,
otherwise the `ic.pipeline` defaults that `ecbench plan` prints
(`linalg = incremental-gauss`, `targets = walk`). Only N differs.
`n90` is not a candidate: a winner picked among several sizes on the same
holdout would be selected on the data that judges it.

## 3. Workloads

Three fresh 20-bit prime-order curves (`prime_search`, seeds 20261029201,
20261029202, 20261029203; slugs `icv1-fp20-tm1319-e428a267`,
`icv1-fp20-tm39-90315544`, `icv1-fp20-tm979-7535758a` per `ecbench plan`),
none of them used by XP-20261016. 32 public targets per curve (target seed
20261029211). Measurement: 1 warm-up and 6 measured rounds, order
`alternate` (balanced with an even round count), seed 20261029221, L2,
timeout 1800 s per execution. 4,704 executions; 96 targets × 6 rounds = 576
matched pairs per arm pair.

## 4. Route

isolab-eks via the AWS launcher (`taskq/deploy/eks-karpenter/remote/launch-remote`),
one job, `c8a.2xlarge`, execution deadline 3600 s, the image's ecbench. A
frozen task card from `experiment-director`, run unchanged by
`executor-mechanical`. Results verified against the job's `SHA256SUMS` after
download. A host loss reruns the whole job under a new job name, recorded.

## 5. Unit and pricing

`S = total GAE / √r`, cold, whole pipeline, as ecbench charges
`ic.pipeline` and `rho.negation`. Every arm, rho included, is priced at
w = 0, 0.1 and 1 GAE per unpriced native operation (AGENTS.md §5a) with
`ecbench compare --weight W` and `ecbench band --ranks`. No calibrated cost
(C = 0 on every arm). Wall time is recorded, never a claim.

## 6. Decision rule (fixed now)

`n64` is an **improvement** and becomes the 20-bit baseline only if all hold:

1. **Instrument:** the session audit (`ecbench verify`) is `ok`;
   `inc-aa/inc` and `rho-neg-aa/rho-neg` are exactly 1.000 at every weight;
   the L2 gate admits every execution.
2. **Completeness:** all 576 measured `n64` and `inc` runs verified
   `[d]G = Q`, 0 censored pairs.
3. **Ratio:** the 95% upper bound of `Σ S_n64(w) / Σ S_inc(w)` (cluster
   bootstrap over targets, 4,000 resamples, seed 20261002) is below 1 at
   w = 0, 0.1 and 1.
4. **Band:** `ecbench band --ranks` gives `n64 < inc` at every weight.

| outcome | reading |
|:--|:--|
| 1–4 hold | improvement; N 64 is the new 20-bit baseline |
| below 1 at some weights, not at others | unresolved inside the band (a trade between charged and native work); baseline kept |
| total falls at w = 0, rises at w = 1 | relabelling; baseline kept |
| interval includes 1 at every weight | null at this sample size; baseline kept |
| upper bound of `Σ S_inc / Σ S_n64` below 1 at every weight | regression; baseline kept |
| 2 fails | incomplete, never a win |

The three curves pool into one paired comparison at one size. Per-curve
means (`ecbench band`) are reported, descriptive only. Always reported:
`inc/rho-neg` and `n64/rho-neg` with intervals (the distance remaining), and
`n90/inc`, `n32/inc` (descriptive).

## 7. Predictions (before any data)

1. `n64/inc` is below 1 at w = 1 (train: 19.46 / 22.10 ≈ 0.88).
2. At w = 0 it is near 1 or above (train at w = 0.1: 11.413 / 11.243 ≈ 1.015),
   so the most likely outcome is "unresolved inside the band".
3. `n32/inc` is above 1 at every weight (reproducing XP-20261016 on new curves).
4. `n90/inc` is above `n64/inc` at w = 0.

## 8. Disclosure

Nothing was run on these curves or seeds before freezing. `ecbench plan`
(validation only, no execution) was run on the spec to read the slugs and
the execution count. The only prior data on N 64 at 20 bits is the
XP-20261016 train cell named above.

## 9. Freezing

Frozen by the commit that adds this file and `spec-holdout.json`; the run's
request embeds `spec-holdout.json` byte for byte (its sha256 is recorded in
the launch card). No amendment after the first execution.

## 10. Outputs

`research/p20_large_n_20261029/`: launch receipts, S3 copy with hashes,
`compare` JSON at three weights for `inc__n64`, `inc__n90`, `inc__n32`,
`inc__inc-aa`, `rho-neg__inc`, `rho-neg__n64`, `rho-neg__rho-neg-aa`, the
band table, a README with figure and PDF. A leaderboard update only on an
improvement that also passes the validation gate.
