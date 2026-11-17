# Finite-base solver diagnostic outcomes

Run `solver-20261021T2349Z-v2`; source freeze `5d6cd4dc595020077d189e999fcbbb6ca64e6ded`. Host `a81fcf6dc864`, Debian 13 x86_64, managed four-CPU virtual host, L0. Workloads use public synthetic fixtures or public-parameter checks. No wall-clock or full-cost improvement is admitted.

Raw outputs: [run directory](../../../execution-v1/runs/solver-20261021T2349Z-v2/). Commands, binary/input hashes, seeds, resource limits and statuses are in [execution receipts](../../../execution-v1/) and the versioned protocols in the study root. No independent-host replay is claimed.

## Recorded observations

| Metric | Count / recorded value |
|:--|--:|
| Singular dimensions returned | 970 |
| cell execution or verification errors | 0 |
| external attempt invalid | false |
| native complete | 731 |
| native ideal equalities verified | 868 |
| native record missing or nonzero | 244 |
| projected set equivalence cells | 240 |
| projected set mismatches | 0 |
| solver cells | 1930 |

![Recorded count outcomes; the editable data are PLOT_DATA.json.](METRICS.pdf)

V2 uses valid ASCII input but the 768 MiB virtual-address limit crashes msolve even on a tiny valid system. Its intended msolve comparison is unresolved. Native and Singular outcomes retain only their separately verified scope. V3 changes every engine to 4 GiB and keeps both earlier attempts.

The chart has no statistical interval: census counts are exact on the stated finite sets; solver completion counts describe these bounded processes only. Missing or failed computations stay unresolved. Full original acceptance criteria and proof/measurement gaps remain in [the requirement matrix](../../../EXPERIMENTS.md).
