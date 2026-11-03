#set page(paper: "a4", margin: 1.7cm)
#set text(size: 9.3pt)
#set table(stroke: 0.4pt)
#show table: set text(size: 8pt)
= Run report: ecbench session `ECBS1h31e0fab46e23` (e-m16.attempt3-killed), 2026-10-19T16:11Z

Part E, `m = 16` (killed after 3 of 4 runs). Spec `specs/SPEC-E-m16.json`. Binary sha256 `c143cf62…`, commit `e403626cb` (clean). Host: Xeon \@ 2.10 GHz, 4 vCPU, one core per session (isolation level L1 or L0, recorded per run: wall time is descriptive only). The session was killed by a container restart: 3 of 4 planned runs were recorded and the rest are missing. Its status is `running`, `ecbench verify` fails exactly its `session\_complete` and `record\_count` checks, and its records were not replayed.

#table(columns: 9, align: (left, left, right, left, right, right, right, right, right),
  [arm], [workload], [$m$], [status], [$S$], [GAE], [trials], [relations], [wall (h)],
  [rho-neg], [`Wa1b3b10e6968`], [16], [verified], [0.8992], [2.731e+09], [–], [–], [0.589],
  [ic-wied], [`Wa1b3b10e6968`], [16], [verified], [1.482], [4.5e+09], [3108064], [16019], [3.077],
  [rho-neg], [`W37a8388076b0`], [16], [verified], [2.235], [6.789e+09], [–], [–], [1.475],
)

All values are recorded fields (`S` = `cost.s`, GAE = `cost.total_gae`, trials and relations from `counters`, wall = `time.solve_wall_ns` ÷ 3.6·10¹²); no run here is a replay. Records: `sessions/e-m16.attempt3-killed/records.jsonl`. Audit receipts: `audits/`.

#figure(image("runs.svg", width: 92%), caption: [Recorded `S` per run, session `ECBS1h31e0fab46e23`.])
