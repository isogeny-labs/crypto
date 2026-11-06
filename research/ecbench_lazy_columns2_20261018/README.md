# Two orbit columns under the lazy member index (2026-10-18)

**Result (engineering, measured):** `columns = 2` is cheaper than the per-curve
column count frozen on 2026-10-14 on **one** of five curves — n = 17, a = 1
(frozen c = 1): 1.176 [1.063, 1.297] — and dearer on the other four (n13
0.637, n23 0.778, n29 a1 0.644, n37 0.246; ratios `S(frozen c) / S(c = 2)`,
paired by target, 95 % CI, counted fully priced GAE). The columns chosen under
the eager oracle stay the better choice on four of five curves once the oracle is
lazy. All 2 880 records verified, 2 160 replays reproduced.

| file | what |
|:--|:--|
| [`REPORT.pdf`](REPORT.pdf) / [`REPORT.typ`](REPORT.typ) | results, graphs, scope and limits |
| [`PROTOCOL.md`](PROTOCOL.md) | frozen before the data (commit `760c672a5`) |
| [`reports/run-20261018T1800Z/`](reports/run-20261018T1800Z/) | the run report |
| `sessions/`, `audits/`, `tables/`, `logs/`, `figures/` | 5 ecbench sessions, replay audits, tables, logs, plotting source and graphs |
| [`../../benchmarks/20261018-lazy-columns2/`](../../benchmarks/20261018-lazy-columns2/) | PDF, SVG and PNG copies (its `benchmarks/README.md` row is withheld, see below) |
| [`compare_rho_frob.sh`](compare_rho_frob.sh) | read-out added after the report: each IC arm against `rho-frob` of the same recorded session (`ecbench compare --save`; no new measurement) |
| [`figures/make_ratios.sh`](figures/make_ratios.sh), `figures/ratios.json` | the saved comparisons gathered for the leaderboard section, the scoreboard panel and the progress point |

Stacked on PR #2158 (the lazy oracle). A per-curve choice of c after this run is
a tuning result on these seeds; it needs fresh seeds before it is a rule.
Dashboards (AGENTS.md §7, added with the owner's go-ahead): a leaderboard component
section (`scripts/build_ic_leaderboard.py`), a scoreboard panel and table row, and one
progress-chart point. The point is the round's best cell against `rho-frob`, the frozen
`c = 1` arm at n = 29 a1 (1.287 [0.971, 1.686], a second measurement of the lazy point
on fresh seeds), not the one improving cell: n17 a1 at `c = 2` (2.152 -> 1.830 against
`rho-frob`) is a tuning gain that does not move the best cell, and a later, higher point
would read as a regression on a polyline chart. The improving cell is in the point's
detail, the leaderboard section and the panel.

Validation standing: not validated under the 2026-10-19 gate (`docs/VALIDATION_GATE.md`).
No `VALIDATION.json` exists for this directory (no replay certificate, validator report
or §5a-banded matched-rho reference is filed), so its `benchmarks/README.md` row is
withheld; the benchmarks directory with the PDF and graphs stays. The dashboard sections
are components outside the gate's board rows and say so.

Reproduce: `bash make_specs.sh` (rewrites the specs byte for byte), `bash run.sh`.
