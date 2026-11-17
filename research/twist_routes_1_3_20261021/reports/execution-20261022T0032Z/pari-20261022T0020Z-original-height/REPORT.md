# Equal-original-height rational-lift control

Run `pari-20261022T0020Z-original-height`; source freeze `0145df8ff32a104d186c31eca2f04c9ee256b21d`. Host `a81fcf6dc864`, Debian 13 x86_64, managed four-CPU virtual host, L0. Workloads use public synthetic fixtures or public-parameter checks. No wall-clock or full-cost improvement is admitted.

Raw outputs: [run directory](../../../execution-v1/runs/pari-20261022T0020Z-original-height/). Commands, binary/input hashes, seeds, resource limits and statuses are in [execution receipts](../../../execution-v1/) and the versioned protocols in the study root. No independent-host replay is claimed.

## Recorded observations

| Metric | Count / recorded value |
|:--|--:|
| rank intervals unresolved | 1 |
| relations outside same base Fp census | 0 |
| same E reductions checked | 42 |
| searches with rational points | 76 |
| searches with three term relations | 7 |
| twist searches | 254 |

![Recorded count outcomes; the editable data are PLOT_DATA.json.](METRICS.pdf)

The same 254 twist parameters are searched at original x height 16 using enclosing standard-model height 16|d| and an exact original-height filter. This is a distinct height domain from V1. Every square-d reduction and projected triple is replayed in the ordinary group on the identical signed base. No row lies outside that same-base census. General target lifting, a complete Mordell-Weil certificate and a full-cost IC comparison remain open.

The chart has no statistical interval: census counts are exact on the stated finite sets; solver completion counts describe these bounded processes only. Missing or failed computations stay unresolved. Full original acceptance criteria and proof/measurement gaps remain in [the requirement matrix](../../../EXPERIMENTS.md).
