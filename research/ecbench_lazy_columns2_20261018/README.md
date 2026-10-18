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
| [`../../benchmarks/20261018-lazy-columns2/`](../../benchmarks/20261018-lazy-columns2/) | PDF, SVG and PNG copies |

Stacked on PR #2158 (the lazy oracle). A per-curve choice of c after this run is
a tuning result on these seeds; it needs fresh seeds before it is a rule.
Not done here: the progress-chart point, the leaderboard section and the
scoreboard panel (AGENTS.md §7) — they depend on #2158's dashboard changes
landing first.

Reproduce: `bash make_specs.sh` (rewrites the specs byte for byte), `bash run.sh`.
