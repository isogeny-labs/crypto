#set page(paper: "a4", margin: 1.7cm)
#set text(size: 9.3pt)
#set table(stroke: 0.4pt)
#show table: set text(size: 8pt)
= Run report: ecbench session `ECBS1h7297c431c21a` (e-m17-ic.attempt1-killed), 2026-10-15T17:44Z

Part E, `m = 17`, the IC half (killed after 1 of 2 runs). Spec `specs/SPEC-E-m17-ic.json`. Binary sha256 `c143cf62…`, commit `e403626cb` (clean). Host: Xeon \@ 2.10 GHz, 4 vCPU, one core per session (isolation level L1 or L0, recorded per run: wall time is descriptive only). The session was killed by a container restart: 1 of 2 planned runs were recorded and the rest are missing. Its status is `running`, `ecbench verify` fails exactly its `session\_complete` and `record\_count` checks, and its records were not replayed.

#table(columns: 9, align: (left, left, right, left, right, right, right, right, right),
  [arm], [workload], [$m$], [status], [$S$], [GAE], [trials], [relations], [wall (h)],
  [ic-wied], [`Wd5f7dba84d0e`], [17], [verified], [0.8493], [1.032e+10], [6173936], [32252], [7.518],
)

All values are recorded fields (`S` = `cost.s`, GAE = `cost.total_gae`, trials and relations from `counters`, wall = `time.solve_wall_ns` ÷ 3.6·10¹²); no run here is a replay. Records: `sessions/e-m17-ic.attempt1-killed/records.jsonl`. Audit receipts: `audits/`.

#figure(image("runs.svg", width: 92%), caption: [Recorded `S` per run, session `ECBS1h7297c431c21a`.])
