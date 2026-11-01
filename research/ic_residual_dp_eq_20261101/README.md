# Priced DP residual collectors (dp and equivariant): result

Protocol [PROTOCOL.md](PROTOCOL.md) (frozen in commit 0fb916d99f, before data). Report: [REPORT.pdf](REPORT.pdf) / [REPORT.html](REPORT.html).
Reproduce: `scripts/make_specs.sh; scripts/run_all.sh; scripts/analyse.sh; python3 scripts/summarise.py`.
Sessions: 2 x 704 records, all verified, `ecbench verify --replay 12` audits OK, A/A 1.0000. Supersedes the w > 0 figures of `../ic_residual_dp_20261101`
(its per-step DP test was unpriced).

C1 (12 (curve, B, candidate) cells, all intervals below 1 at w = 0, 0.1, 1 in the band as built): **not met**. 9 of 12 cells hold: eq 6/6, dp 3/6 (the three failures are at w = 0).
Candidate/baseline counted S: eq 0.64-0.68 (w = 0), 0.24-0.36 (w = 0.1), 0.09-0.16 (w = 1); dp 0.82-0.94, 0.28-0.43, 0.07-0.13.
Distance to matched rho (w = 0 / 0.1 / 1): baseline res4 ~4.0 / 12.7 / 51.9; best arms res1eq 1.54 / 1.75 / 2.66 (p36), 1.64 / 1.86 / 2.83 (p37); res1dp 2.13 / 2.14 / 2.22 and 2.03 / 2.04 / 2.07.
eq vs dp: eq lower at w = 0, dp lower at w = 1; order unresolved inside the band. Class: engineering of a generic collision search; rho stays ahead everywhere. No VALIDATION.json: no leaderboard / benchmarks row.


> **Pricing caveat and errata (added after the independent validator's report, `benchmarks/20261101-ic-residual-dp-eq/validator-report.json`).** (1) The band as built prices `residual_table_bytes_uncharged`, a byte count, as operations at w > 0; the large w > 0 gains and the 48-53x baseline distances below are mostly that artifact. With that counter excluded: dp/baseline is a constant 0.76-0.94 at every weight, eq/baseline 0.54-0.68 at w = 0 and 0.9-1.18 at w = 1, and the baseline's distance to rho is 2.5-4.1x at every weight. w = 0 results are unaffected. See REPORT.pdf. (2) The C1 tally is 9 of 12 (curve, B, candidate) cells (eq 6/6, dp 3/6), not "12/12 and 9/12 of 24"; the protocol text's "24 cells" was an arithmetic slip.
