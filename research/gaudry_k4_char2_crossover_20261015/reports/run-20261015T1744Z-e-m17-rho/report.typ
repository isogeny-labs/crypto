#set page(paper: "a4", margin: 1.7cm)
#set text(size: 9.3pt)
#set table(stroke: 0.4pt)
#show table: set text(size: 8pt)
= Run report: ecbench session `ECBS1hbfd18e44ce13` (e-m17-rho), 2026-10-15T17:44Z

Part E, `m = 17`, the rho half. Spec `specs/SPEC-E-m17-rho.json`. Binary sha256 `c143cf62…`, commit `e403626cb` (clean). Host: Xeon \@ 2.10 GHz, 4 vCPU, one core per session (isolation level L1 or L0, recorded per run: wall time is descriptive only). The session completed: every planned run was recorded.

#table(columns: 9, align: (left, left, right, left, right, right, right, right, right),
  [arm], [workload], [$m$], [status], [$S$], [GAE], [trials], [relations], [wall (h)],
  [rho-neg], [`Wd5f7dba84d0e`], [17], [verified], [1.361], [1.654e+10], [–], [–], [4.510],
  [rho-neg], [`W652ef8c5cfb7`], [17], [verified], [0.9977], [1.212e+10], [–], [–], [3.005],
)

All values are recorded fields (`S` = `cost.s`, GAE = `cost.total_gae`, trials and relations from `counters`, wall = `time.solve_wall_ns` ÷ 3.6·10¹²); no run here is a replay. Records: `sessions/e-m17-rho/records.jsonl`. Audit receipts: `audits/`.

#figure(image("runs.svg", width: 92%), caption: [Recorded `S` per run, session `ECBS1hbfd18e44ce13`.])
