# Pairing-control first attempt: no emitted result

Run `pairing-20261022T0020Z`; source freeze `0145df8ff32a104d186c31eca2f04c9ee256b21d`. Host `a81fcf6dc864`, Debian 13 x86_64, managed four-CPU virtual host, L0. Workloads use public synthetic fixtures or public-parameter checks. No wall-clock or full-cost improvement is admitted.

Raw outputs: [run directory](../../../execution-v1/runs/pairing-20261022T0020Z/). Commands, binary/input hashes, seeds, resource limits and statuses are in [execution receipts](../../../execution-v1/) and the versioned protocols in the study root. No independent-host replay is claimed.

## Recorded observations

| Metric | Count / recorded value |
|:--|--:|
| GP exit code | 0 |
| accepted independent logs | 0 |
| parsed result | false |
| public target controls | 10 |

![Recorded count outcomes; the editable data are PLOT_DATA.json.](METRICS.pdf)

The GP function definition consumed its output statements on the same line. GP exited zero with no result. This attempt is unresolved and supplies no independent pairing-validation claim. V2 isolates the function definition and retries the identical inputs in a new directory.

The chart has no statistical interval: census counts are exact on the stated finite sets; solver completion counts describe these bounded processes only. Missing or failed computations stay unresolved. Full original acceptance criteria and proof/measurement gaps remain in [the requirement matrix](../../../EXPERIMENTS.md).
