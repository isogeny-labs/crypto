# Conditional homomorphism and integral-lift ingress

Run `conditional-20261021T2256Z`; source freeze `d86eb011034066532d3645369a783f44be158b1c`. Host `a81fcf6dc864`, Debian 13 x86_64, managed four-CPU virtual host, L0. Workloads use public synthetic fixtures or public-parameter checks. No wall-clock or full-cost improvement is admitted.

Raw outputs: [run directory](../../../execution-v1/runs/conditional-20261021T2256Z/). Commands, binary/input hashes, seeds, resource limits and statuses are in [execution receipts](../../../execution-v1/) and the versioned protocols in the study root. No independent-host replay is claimed.

## Recorded observations

| Metric | Count / recorded value |
|:--|--:|
| gcd one | 15 |
| public parameter cases | 15 |
| r primary hom applicable | 0 |

![Recorded count outcomes; the editable data are PLOT_DATA.json.](METRICS.pdf)

NIST P-curve checks use public parameters only: gcd(r,#twist)=1 and no embedding degree found through the recorded bound 1,024. A bound is not the exact degree. The small integral lift box is supplemented, not replaced, by the separate PARI rational-height run. No NIST DLP, pairing transfer or scalable lift attack was run.

The chart has no statistical interval: census counts are exact on the stated finite sets; solver completion counts describe these bounded processes only. Missing or failed computations stay unresolved. Full original acceptance criteria and proof/measurement gaps remain in [the requirement matrix](../../../EXPERIMENTS.md).
