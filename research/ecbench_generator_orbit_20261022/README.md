# Orbit-layout pair table and generator-orbit base (2026-10-22)

Engineering study against the lazy orbit IC baseline (PR #2158). The
protocol, [`PROTOCOL.md`](PROTOCOL.md), was frozen in `d2b3a56c32` before any
data. The report is [`REPORT.pdf`](REPORT.pdf) (source `REPORT.typ`).

| what | where |
|:--|:--|
| specs | `docs/ecbench/specs/claim-koblitz-generator-orbit-a*-n*.json` (from `make_specs.sh`) |
| run | `run.sh`, executed through the task card `cards/CARD-ecbench-generator-orbit-20261022-2.yaml` (director → mechanical relay; receipt and assessment beside it; attempt 1 in `cards/attempt-1/`) |
| sessions | `sessions/<a?-n??>/` (7 ecbench sessions; `comparisons/` holds the saved paired comparisons) |
| audits | `audits/audit-*.json` (`verify --replay-all`) |
| tables | `tables/table-*.md`, `tables/band-*.{md,json}` (§5a band), `tables/compare-*.txt` |
| H1 | `check_same_algorithm.sh` (lazy = layout on trials, relations, rank, log, entries) |
| figures | `figures/make_summary.sh` → `figures/summary.json` → `figures/plot_figures.py` → `fig*.svg/png` |
| validation | `validation/` (replay certificate, validator report), `VALIDATION.json` |

## Result in one paragraph

- **Layout table.** On all 7 Koblitz curves (n13–n37) the orbit-layout table
  finds the same relations as the lazy table (960/960 run pairs identical) and
  costs 1.02×–1.09× less.
- **Generator orbit.** With the generator-orbit base and the generator stride
  added, the net gain over the baseline is 1.26×–3.20×, the lower 95 % end above
  1 on every curve. The band order holds at w = 0, 0.1 and 1.
- **n29, against rho.** The IC arm is below the matched rho-frob at every band
  weight: S 0.546 / 0.611 / 1.196 against 1.569, paired ratio 0.366
  [0.241, 0.523].
- **What that crossing is.** At one column the pipeline is a Frobenius-folded
  baby-step giant-step, and the crossing rests on rho-frob's fixed 133-GAE
  set-up at log₂ r = 15.4. The gap to rho grows with r: 1.15× at n31, 1.93× at
  n23, 3.89× at n37.
- **Not shown.** At n19, which has no pinned calibration row, the band does not
  re-price the IC natives.
