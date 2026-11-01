# Priced DP residual collectors (dp and equivariant): result

Protocol [PROTOCOL.md](PROTOCOL.md) (frozen in commit 0fb916d99f, before data). Report: [REPORT.pdf](REPORT.pdf) / [REPORT.html](REPORT.html).
Reproduce: `scripts/make_specs.sh; scripts/run_all.sh; scripts/analyse.sh; python3 scripts/summarise.py`.
Sessions: 2 x 704 records, all verified, `ecbench verify --replay 12` audits OK, A/A 1.0000. Supersedes the w > 0 figures of `../ic_residual_dp_20261101`
(its per-step DP test was unpriced).

C1 (24 cells: dp and eq vs baseline, B = 1, 2, 4, p36 and p37, all intervals below 1 at w = 0, 0.1, 1): **not met**. eq 12/12 cells hold; dp 9/12 (the three failures are at w = 0).
Candidate/baseline counted S: eq 0.64-0.68 (w = 0), 0.24-0.36 (w = 0.1), 0.09-0.16 (w = 1); dp 0.82-0.94, 0.28-0.43, 0.07-0.13.
Distance to matched rho (w = 0 / 0.1 / 1): baseline res4 ~4.0 / 12.7 / 51.9; best arms res1eq 1.54 / 1.75 / 2.66 (p36), 1.64 / 1.86 / 2.83 (p37); res1dp 2.13 / 2.14 / 2.22 and 2.03 / 2.04 / 2.07.
eq vs dp: eq lower at w = 0, dp lower at w = 1; order unresolved inside the band. Class: engineering of a generic collision search; rho stays ahead everywhere. No VALIDATION.json: no leaderboard / benchmarks row.
