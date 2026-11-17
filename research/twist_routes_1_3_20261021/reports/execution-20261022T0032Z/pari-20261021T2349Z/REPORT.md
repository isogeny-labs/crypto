# Independent PARI geometry and bounded rational-twist searches

Run `pari-20261021T2349Z`; source freeze `5d6cd4dc595020077d189e999fcbbb6ca64e6ded`. Host `a81fcf6dc864`, Debian 13 x86_64, managed four-CPU virtual host, L0. Workloads use public synthetic fixtures or public-parameter checks. No wall-clock or full-cost improvement is admitted.

Raw outputs: [run directory](../../../execution-v1/runs/pari-20261021T2349Z/). Commands, binary/input hashes, seeds, resource limits and statuses are in [execution receipts](../../../execution-v1/) and the versioned protocols in the study root. No independent-host replay is claimed.

## Recorded observations

| Metric | Count / recorded value |
|:--|--:|
| independent cover characteristic polynomials | 10 |
| rank intervals unresolved | 1 |
| relations outside same base Fp census | 0 |
| same E reductions checked | 12 |
| searches with rational points | 25 |
| searches with three term relations | 5 |
| twist searches | 254 |

![Recorded count outcomes; the editable data are PLOT_DATA.json.](METRICS.pdf)

PARI 2.17.2 independently computes all ten selected-cover characteristic polynomials. The 254 rational searches use d=-64..64 (excluding zero and bad reduction), rank effort zero, and standard-twist x height 16. Rust checks square-d finite-field reductions and every projected three-term relation against the identical-base ordinary census. Rank bounds are not a formal Mordell-Weil proof; standard-model height is not equal original-model height. General rational target lifting and all discovery/height costs remain open.

The chart has no statistical interval: census counts are exact on the stated finite sets; solver completion counts describe these bounded processes only. Missing or failed computations stay unresolved. Full original acceptance criteria and proof/measurement gaps remain in [the requirement matrix](../../../EXPERIMENTS.md).
