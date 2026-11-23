#set page(paper: "a4", margin: 1.7cm)
#set text(size: 9.3pt)
#set table(stroke: 0.4pt)
#show table: set text(size: 8pt)
= Run report: ecbench session `ECBS1h5394ca2448f0` (e-m17-ic-t0), 2026-10-27T00:11Z

Part E, `m = 17`, IC on target 0 (checkpointing binary). Spec `specs/SPEC-E-m17-ic-t0.json`. Binary sha256 `ad1dd91a…`, commit `2cdffcf22` (clean). Host: Xeon \@ 2.10 GHz, 4 vCPU, one core per session (isolation level L1 or L0, recorded per run: wall time is descriptive only). The session completed: every planned run was recorded.

#table(columns: 9, align: (left, left, right, left, right, right, right, right, right),
  [arm], [workload], [$m$], [status], [$S$], [GAE], [trials], [relations], [wall (h)],
  [ic-wied], [`Wd5f7dba84d0e`], [17], [verified], [0.8416], [1.022e+10], [6117400], [32242], [3.699],
)

All values are recorded fields (`S` = `cost.s`, GAE = `cost.total_gae`, trials and relations from `counters`, wall = `time.solve_wall_ns` ÷ 3.6·10¹²); no run here is a replay. Records: `sessions/e-m17-ic-t0/records.jsonl`. Audit receipts: `audits/`.

#figure(image("runs.svg", width: 92%), caption: [Recorded `S` per run, session `ECBS1h5394ca2448f0`.])
