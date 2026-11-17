# Explicit low-degree isogeny and return-map controls

Run `isogeny-20261021T2320Z`; source freeze `411a254c0ac518e8462ecd6f07e4f315917a48b6`. Host `a81fcf6dc864`, Debian 13 x86_64, managed four-CPU virtual host, L0. Workloads use public synthetic fixtures or public-parameter checks. No wall-clock or full-cost improvement is admitted.

Raw outputs: [run directory](../../../execution-v1/runs/isogeny-20261021T2320Z/). Commands, binary/input hashes, seeds, resource limits and statuses are in [execution receipts](../../../execution-v1/) and the versioned protocols in the study root. No independent-host replay is claimed.

## Recorded observations

| Metric | Count / recorded value |
|:--|--:|
| algebraic dual maps | 2 |
| degree fixture cells | 30 |
| explicit maps | 3 |
| inapplicable | 27 |

![Recorded count outcomes; the editable data are PLOT_DATA.json.](METRICS.pdf)

Degrees 2, 3 and 7 were checked on ten fixtures. Eligible kernels are constructed over the frozen Fp⁶ tower; the rational functions descend to Fp. Every rational source point, generator addition and available dual composition is checked. The finite lookup return map has linear-in-group-order setup and supplies no scalable DLP gain. New destination representations are registered separately.

The chart has no statistical interval: census counts are exact on the stated finite sets; solver completion counts describe these bounded processes only. Missing or failed computations stay unresolved. Full original acceptance criteria and proof/measurement gaps remain in [the requirement matrix](../../../EXPERIMENTS.md).

![Verified source/destination models and return maps. A dashed return uses a finite lookup table; no volcano level is inferred.](MAPS.pdf)
