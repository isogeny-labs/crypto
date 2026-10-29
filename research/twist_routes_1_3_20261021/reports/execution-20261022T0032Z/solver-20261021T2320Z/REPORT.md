# Finite-base solver diagnostic outcomes

Run `solver-20261021T2320Z`; source freeze `411a254c0ac518e8462ecd6f07e4f315917a48b6`. Host `a81fcf6dc864`, Debian 13 x86_64, managed four-CPU virtual host, L0. Workloads use public synthetic fixtures or public-parameter checks. No wall-clock or full-cost improvement is admitted.

Raw outputs: [run directory](../../../execution-v1/runs/solver-20261021T2320Z/). Commands, binary/input hashes, seeds, resource limits and statuses are in [execution receipts](../../../execution-v1/) and the versioned protocols in the study root. No independent-host replay is claimed.

## Recorded observations

| Metric | Count / recorded value |
|:--|--:|
| Singular dimensions returned | 0 |
| cell execution or verification errors | 458 |
| external attempt invalid | true |
| native complete | 558 |
| native ideal equalities verified | 0 |
| native record missing or nonzero | 701 |
| projected set equivalence cells | 240 |
| projected set mismatches | 0 |
| solver cells | 1930 |

![Recorded count outcomes; the editable data are PLOT_DATA.json.](METRICS.pdf)

The V1 external attempt is INVALID: Unicode operators were sent to the CAS tools. Reject every external comparison from this attempt. Native outputs retain their own scope. Preserve all raw cases in the deterministic split archive; do not count failure as refutation.

The chart has no statistical interval: census counts are exact on the stated finite sets; solver completion counts describe these bounded processes only. Missing or failed computations stay unresolved. Full original acceptance criteria and proof/measurement gaps remain in [the requirement matrix](../../../EXPERIMENTS.md).
