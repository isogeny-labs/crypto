# Repeated-eigenvalue dual-map completion

Run `isogeny-20261022T0054Z-v2`; source freeze `1284090fd4db2f55e023afcb275d94f3ce691b10`. Host `a81fcf6dc864`, Debian 13 x86_64, managed four-CPU virtual host, L0. Workloads use public synthetic fixtures or public-parameter checks. No wall-clock or full-cost improvement is admitted.

Raw outputs: [run directory](../../../execution-v1/runs/isogeny-20261022T0054Z-v2/). Commands, binary/input hashes, seeds, resource limits and statuses are in [execution receipts](../../../execution-v1/) and the versioned protocols in the study root. No independent-host replay is claimed.

## Recorded observations

| Metric | Count / recorded value |
|:--|--:|
| algebraic dual maps | 3 |
| degree fixture cells | 30 |
| dual kernel searches | 3 |
| explicit maps | 3 |
| scalar recovery controls | 24 |

![Recorded count outcomes; the editable data are PLOT_DATA.json.](METRICS.pdf)

The repeated-eigenvalue case samples another order-ell torsion point and requires its forward image to be nonzero; an eigenvalue alone is not accepted as an independent line. All three eligible degree-seven maps now have explicit algebraic duals, composition [7] checked on every rational source point, and eight scalar-recovery controls each. The original 30 rows, including its missing dual, remain intact. These exact toy maps supply no IC or NIST speed comparison; full cost and formal map bridges remain open.

The chart has no statistical interval: census counts are exact on the stated finite sets; solver completion counts describe these bounded processes only. Missing or failed computations stay unresolved. Full original acceptance criteria and proof/measurement gaps remain in [the requirement matrix](../../../EXPERIMENTS.md).

![Verified source/destination models and return maps. A dashed return uses a finite lookup table; no volcano level is inferred.](MAPS.pdf)
