# Frozen synthetic fixture preparation

Run `fixture-preparation`; source freeze `d86eb011034066532d3645369a783f44be158b1c`. Host `a81fcf6dc864`, Debian 13 x86_64, managed four-CPU virtual host, L0. Workloads use public synthetic fixtures or public-parameter checks. No wall-clock or full-cost improvement is admitted.

Raw outputs: [run directory](../../../execution-v1/). Commands, binary/input hashes, seeds, resource limits and statuses are in [execution receipts](../../../execution-v1/) and the versioned protocols in the study root. No independent-host replay is claimed.

## Recorded observations

| Metric | Count / recorded value |
|:--|--:|
| generated ladder fixtures | 8 |
| registered fixture models | 13 |
| rejected ladder candidates retained | 147 |

![Recorded count outcomes; the editable data are PLOT_DATA.json.](METRICS.pdf)

The native preparation freezes 13 exact curve/twist/extension fixtures, including eight A=-3 ordinary prime-order ladder models. Public SplitMix seed 20261021 plus declared size/replicate offsets, every rejected candidate, exact whole/subgroup orders, generators and tower-to-flat conversions are retained. This is fixture construction, not a scaling or DLP performance result. Identities are independently reconstructed by the repository's canonical Rust constructor in registry-check-v2.json.

The chart has no statistical interval: census counts are exact on the stated finite sets; solver completion counts describe these bounded processes only. Missing or failed computations stay unresolved. Full original acceptance criteria and proof/measurement gaps remain in [the requirement matrix](../../../EXPERIMENTS.md).
