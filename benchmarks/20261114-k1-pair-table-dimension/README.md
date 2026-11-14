# K1 pair-table factor base sized for n against dimension 7 (2026-11-14)

Study: [`research/ml_cryptanalysis_pt_lgrow_k1_20261114/`](../../research/ml_cryptanalysis_pt_lgrow_k1_20261114/README.md)
(frozen copy of ml-cryptanalysis `experiments/fb-pt-lgrow-k1` at `7d4e8e0f`; evidence directory
`fb-pt-lgrow-k1/`, validation `VAL-20261114-d9190c`). These rows reached the IC leaderboard in PR #4149.
Nothing was re-run for this entry: the numbers come from the study's frozen `summary.json`.

| curve | n | r | dimension | gap to strong rho, bits (w = 0 / 0.1 / 1) | dimension 7 |
|:--|--:|:--|--:|:--|:--|
| `icv1-f2m23-tm5197-1f85e9e1` | 23 | 2^22.00 | 9 | 5.59 / 5.66 / 5.95 | 6.91 / 6.94 / 7.09 |
| `icv1-f2m29-tm40309-30c52b96` | 29 | 2^15.37 (cofactor 12646) | 10 | 9.47 / 9.52 / 9.86 | 10.36 / 10.46 / 11.06 |

One target, cold; ecbench counted GAE with native work at w GAE (AGENTS.md §5a); 8 targets × 5 rounds,
40 verified solves an arm; all arms in one ecbench session (sha256 `a0be1b2a…8796d29`), one RunPod CPU
pod, L0. This is an engineering improvement over dimension 7 at fixed n. Rho is cheaper than both rows at
every weight. Claim `PCC-20261114-8ba038` [pcc:unverified].

Files:

- `REPORT.pdf` (source `report.typ`): the report.
- `gap.svg` and `gap.png` (source `plot.py`, which reads `summary.json` and only plots): gap to rho before and after, with 95% intervals.
- `fig_lgrow_gap.svg` and `fig_lgrow_gap.png`: unchanged copies of the study figure (gap against n = 17, 19, 23, 29).
- `VALIDATION.json`: pins the study's replay certificate and `VALIDATOR.md` in place, and copies its `rho_reference` unchanged.
- `validator-report.json`: a copy of the study's validator report with only the `raw_data` paths rebased to this directory. The `path_rebase` field names the source file and its sha256.
- Record: `benchmarks/records/` entry naming this directory.
