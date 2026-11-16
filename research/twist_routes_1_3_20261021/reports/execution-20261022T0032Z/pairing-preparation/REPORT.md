# Separate pairing-positive fixture preparation

Run `pairing-preparation`; source freeze `0145df8ff32a104d186c31eca2f04c9ee256b21d`. Host `a81fcf6dc864`, Debian 13 x86_64, managed four-CPU virtual host, L0. Workloads use public synthetic fixtures or public-parameter checks. No wall-clock or full-cost improvement is admitted.

Raw outputs: [run directory](../../../execution-v1/pairing-fixture/). Commands, binary/input hashes, seeds, resource limits and statuses are in [execution receipts](../../../execution-v1/) and the versioned protocols in the study root. No independent-host replay is claimed.

## Recorded observations

| Metric | Count / recorded value |
|:--|--:|
| extension points | 1936 |
| known target controls | 10 |
| source points | 44 |
| source subgroup order | 11 |

![Recorded count outcomes; the editable data are PLOT_DATA.json.](METRICS.pdf)

The native preparation exhaustively counts the rational and quadratic-extension groups of the separately registered supersingular F43 model, chooses a nonidentity order-11 source generator, checks the distortion point lies outside that cyclic subgroup, and records ten known-scalar controls. This fixture is separate from the ordinary/NIST study. Independent PARI and source replay occur in the later V2 control.

The chart has no statistical interval: census counts are exact on the stated finite sets; solver completion counts describe these bounded processes only. Missing or failed computations stay unresolved. Full original acceptance criteria and proof/measurement gaps remain in [the requirement matrix](../../../EXPERIMENTS.md).
