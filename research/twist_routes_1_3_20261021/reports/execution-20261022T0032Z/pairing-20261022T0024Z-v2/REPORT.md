# Known small-degree pairing positive control

Run `pairing-20261022T0024Z-v2`; source freeze `e517e80b1e77778d529f51ce5a900e00cb010761`. Host `a81fcf6dc864`, Debian 13 x86_64, managed four-CPU virtual host, L0. Workloads use public synthetic fixtures or public-parameter checks. No wall-clock or full-cost improvement is admitted.

Raw outputs: [run directory](../../../execution-v1/runs/pairing-20261022T0024Z-v2/). Commands, binary/input hashes, seeds, resource limits and statuses are in [execution receipts](../../../execution-v1/) and the versioned protocols in the study root. No independent-host replay is claimed.

## Recorded observations

| Metric | Count / recorded value |
|:--|--:|
| embedding degree | 2 |
| independent PARI logs | 10 |
| known scalar positive control only | true |
| native bilinearity controls | 10 |
| source group replay | true |
| subgroup order | 11 |

![Recorded count outcomes; the editable data are PLOT_DATA.json.](METRICS.pdf)

This separate supersingular control uses the registered F43 model and F43² with w²+1, r=11 and a distortion point outside the source cyclic subgroup. Native Miller arithmetic and independent PARI Tate pairing/finite-field logarithms recover all ten deliberately known scalar controls, checked again on the public source points. The GP script receives only target coordinates. This validates a known positive conditional branch; it is no ordinary/NIST transfer, novelty claim or speed result.

The chart has no statistical interval: census counts are exact on the stated finite sets; solver completion counts describe these bounded processes only. Missing or failed computations stay unresolved. Full original acceptance criteria and proof/measurement gaps remain in [the requirement matrix](../../../EXPERIMENTS.md).
