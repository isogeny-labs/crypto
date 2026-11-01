# Fused residual step + distinguished points: result

Protocol [PROTOCOL.md](PROTOCOL.md) (frozen in commit d692205133, before data). Report: [REPORT.pdf](REPORT.pdf) / [REPORT.html](REPORT.html).
Reproduce: `scripts/make_specs.sh; scripts/run_all.sh; scripts/analyse.sh; python3 scripts/summarise.py`.
Sessions: 4 x 512 records, all verified; `ecbench verify --replay 12` audit OK on each; A/A ratio 1.0000.

Headline (candidate/baseline counted S, DP-active curves p36, p37, B = 2, 4, 8): w = 0: 0.80-0.91; w = 0.1: 0.17-0.33; w = 1: 0.024-0.055.
Frozen rule C1 is met in 5 of 6 cells (p36 res2 at w = 0: [0.747, 1.026]); reported as not met strictly.
Distance to matched rho for res4dp (w = 0 / 0.1 / 1): p36 3.76 / 3.48 / 2.24; p37 3.24 / 2.98 / 1.78 (baseline res4: 4.14 / 13.1 / 53.7 and 3.70 / 11.8 / 48.1).
Class: engineering of a generic collision search (lower priced native work), not structural IC. No VALIDATION.json yet: no leaderboard / benchmarks row.
