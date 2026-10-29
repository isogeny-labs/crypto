# Cold factorial IC pipeline: correctness and accounting limits

Run `ecbench-20261022T0012Z`; source freeze `2bd6cd2e95e139fa67b2b41bea59b43ca217165b`. Host `a81fcf6dc864`, Debian 13 x86_64, managed four-CPU virtual host, L0. Workloads use public synthetic fixtures or public-parameter checks. No wall-clock or full-cost improvement is admitted.

Raw outputs: [run directory](../../../execution-v1/runs/ecbench-20261022T0012Z/). Commands, binary/input hashes, seeds, resource limits and statuses are in [execution receipts](../../../execution-v1/) and the versioned protocols in the study root. No independent-host replay is claimed.

## Recorded observations

| Metric | Count / recorded value |
|:--|--:|
| audit ok | true |
| errors | 216 |
| executions | 1920 |
| full cost ranking admitted | false |
| identical replays | 1704 |
| verified | 1704 |
| wrong answers | 0 |

![Recorded count outcomes; the editable data are PLOT_DATA.json.](METRICS.pdf)

Eight public one-target workloads on each of ten registered curves, 24 arms, one L0 round and no warmup. All returned logarithms are point-verified. The 216 errors are retained empty trace/cover intersections. All 1,704 verified records replay identically. The field/call, F4, symbolic and extraction work is unpriced. The CSV applies 0/0.1/1 GAE to every recorded unpriced native unit on every arm, including rho; this is a sensitivity model, not calibrated full cost. No ranking or speedup is admitted. Native compare/table wall-admission labels under a requested L0 minimum are explicitly excluded by this protocol; no wall figure is a result. Other sizes, arities, independent-host replay and closed peak-memory/accounting remain open.

The chart has no statistical interval: census counts are exact on the stated finite sets; solver completion counts describe these bounded processes only. Missing or failed computations stay unresolved. Full original acceptance criteria and proof/measurement gaps remain in [the requirement matrix](../../../EXPERIMENTS.md).
