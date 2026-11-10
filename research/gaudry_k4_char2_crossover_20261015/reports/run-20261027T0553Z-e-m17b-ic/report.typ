#set page(paper: "a4", margin: 1.7cm)
#set text(size: 9.3pt)
#set table(stroke: 0.4pt)
#show table: set text(size: 8pt)
= Run report: ecbench session `ECBS1hc5c21ea63db6` (e-m17b-ic), 2026-10-27T05:53Z

Part E, `m = 17`, IC on a second target (checkpointing binary). Spec `specs/SPEC-E-m17b-ic.json`. Binary sha256 `ad1dd91a…`, commit `1b15f8b35` (clean). Host: Xeon \@ 2.10 GHz, 4 vCPU, one core per session (isolation level L1 or L0, recorded per run: wall time is descriptive only). The session completed: every planned run was recorded.

#table(columns: 9, align: (left, left, right, left, right, right, right, right, right),
  [arm], [workload], [$m$], [status], [$S$], [GAE], [trials], [relations], [wall (h)],
  [ic-wied], [`W591d11cef5d4`], [17], [verified], [0.8492], [1.032e+10], [6172522], [32192], [11.112],
)

All values are recorded fields (`S` = `cost.s`, GAE = `cost.total_gae`, trials and relations from `counters`, wall = `time.solve_wall_ns` ÷ 3.6·10¹²); no run here is a replay. Records: `sessions/e-m17b-ic/records.jsonl`. Audit receipts: `audits/`.

#figure(image("runs.svg", width: 92%), caption: [Recorded `S` per run, session `ECBS1hc5c21ea63db6`.])
