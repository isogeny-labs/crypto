#set page(paper: "a4", margin: 1.7cm)
#set text(size: 9.3pt)
#set table(stroke: 0.4pt)
#show table: set text(size: 8pt)
= Run report: ecbench session `ECBS1h1c8784cce030` (e-m15), 2026-10-19T16:11Z

Part E, `m = 15`. Spec `specs/SPEC-E-m15.json`. Binary sha256 `c143cf62…`, commit `e403626cb` (clean). Host: Xeon \@ 2.10 GHz, 4 vCPU, one core per session (isolation level L1 or L0, recorded per run: wall time is descriptive only). The session completed: every planned run was recorded.

#table(columns: 9, align: (left, left, right, left, right, right, right, right, right),
  [arm], [workload], [$m$], [status], [$S$], [GAE], [trials], [relations], [wall (h)],
  [rho-neg], [`W588ac37c7439`], [15], [verified], [0.1677], [1.273e+08], [–], [–], [0.019],
  [ic-wied], [`W588ac37c7439`], [15], [verified], [3.198], [2.428e+09], [1545102], [8020], [1.416],
  [rho-neg-aa], [`W588ac37c7439`], [15], [verified], [0.1677], [1.273e+08], [–], [–], [0.019],
  [rho-neg], [`W761fd6d9429f`], [15], [verified], [0.8562], [6.501e+08], [–], [–], [0.097],
  [ic-wied], [`W761fd6d9429f`], [15], [verified], [3.253], [2.47e+09], [1572023], [8004], [1.437],
  [rho-neg-aa], [`W761fd6d9429f`], [15], [verified], [0.8562], [6.501e+08], [–], [–], [0.099],
)

All values are recorded fields (`S` = `cost.s`, GAE = `cost.total_gae`, trials and relations from `counters`, wall = `time.solve_wall_ns` ÷ 3.6·10¹²); no run here is a replay. Records: `sessions/e-m15/records.jsonl`. Audit receipts: `audits/`.

#figure(image("runs.svg", width: 92%), caption: [Recorded `S` per run, session `ECBS1h1c8784cce030`.])
