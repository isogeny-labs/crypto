# Finite-base solver diagnostic outcomes

Run `solver-20261022T0004Z-v3`; source freeze `2bd6cd2e95e139fa67b2b41bea59b43ca217165b`. Host `a81fcf6dc864`, Debian 13 x86_64, managed four-CPU virtual host, L0. Workloads use public synthetic fixtures or public-parameter checks. No wall-clock or full-cost improvement is admitted.

Raw outputs: [run directory](../../../execution-v1/runs/solver-20261022T0004Z-v3/). Commands, binary/input hashes, seeds, resource limits and statuses are in [execution receipts](../../../execution-v1/) and the versioned protocols in the study root. No independent-host replay is claimed.

## Recorded observations

| Metric | Count / recorded value |
|:--|--:|
| Singular dimensions returned | 968 |
| cell execution or verification errors | 0 |
| external attempt invalid | false |
| native complete | 730 |
| native ideal equalities verified | 866 |
| native record missing or nonzero | 196 |
| projected set equivalence cells | 240 |
| projected set mismatches | 0 |
| solver cells | 1930 |

![Recorded count outcomes; the editable data are PLOT_DATA.json.](METRICS.pdf)

V3 uses valid ASCII input and a 4 GiB virtual-address limit for every engine. Cells retain the one-second hard limit (native 500 ms cooperative), one thread and three concurrent cells. Completed bases, verified ideal equalities, unresolved bases and process failures are separate. Counts of kernel multiplications do not price the full solver. B=12/16 and other factor-base families remain outside this initial panel.

The chart has no statistical interval: census counts are exact on the stated finite sets; solver completion counts describe these bounded processes only. Missing or failed computations stay unresolved. Full original acceptance criteria and proof/measurement gaps remain in [the requirement matrix](../../../EXPERIMENTS.md).
