# Trace fibers, restrictions and relation censuses

Run `trace-20261021T2256Z`; source freeze `d86eb011034066532d3645369a783f44be158b1c`. Host `a81fcf6dc864`, Debian 13 x86_64, managed four-CPU virtual host, L0. These are public synthetic small-field records. No wall-clock or full-cost improvement is admitted.

Raw outputs: [run directory](../../../execution-v1/runs/trace-20261021T2256Z/). Commands, binary/input hashes, seeds, resource limits and statuses are in [execution receipts](../../../execution-v1/) and the versioned protocols in the study root. No independent-host replay is claimed.

## Recorded observations

| Metric | Count / recorded value |
|:--|--:|
| divisor fixtures | 2 |
| exhaustive trace fixtures | 9 |
| quartic mismatches | 0 |
| quartic witness equivalence cells | 30 |
| relation census cells | 1179 |

![Recorded count outcomes; the editable data are PLOT_DATA.json.](METRICS.pdf)

E01 is an exhaustive Fp² census for nine fixtures. The rational quartic and extension-coordinate parameterizations give identical witness sets. E02/E04 full larger-field norm-y/power fibers and E05 unsplit degree-three/four divisors remain partial. E06 enumerates finite-base relations, not decomposition time; reused bases and unsupported sizes are retained.

The chart has no statistical interval: census counts are exact on the stated finite sets; solver completion counts describe these bounded processes only. Missing or failed computations stay unresolved. Full original acceptance criteria and proof/measurement gaps remain in [the requirement matrix](../../../EXPERIMENTS.md).
