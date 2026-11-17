#set page(paper: "a4", margin: 1.7cm)
#set text(size: 9.3pt)
#set table(stroke: 0.4pt)
#show table: set text(size: 8pt)
= Run report: ecbench session `ECBS1hd81d73afe25e` (e-m16), 2026-10-27T05:53Z

Part E, `m = 16`, two targets, IC and rho (checkpointing binary). Spec `specs/SPEC-E-m16.json`. Binary sha256 `ad1dd91a…`, commit `1b15f8b35` (clean). Host: Xeon \@ 2.10 GHz, 4 vCPU, one core per session (isolation level L1 or L0, recorded per run: wall time is descriptive only). The session completed: every planned run was recorded.

#table(columns: 9, align: (left, left, right, left, right, right, right, right, right),
  [arm], [workload], [$m$], [status], [$S$], [GAE], [trials], [relations], [wall (h)],
  [rho-neg], [`Wa1b3b10e6968`], [16], [verified], [0.8992], [2.731e+09], [–], [–], [0.815],
  [ic-wied], [`Wa1b3b10e6968`], [16], [verified], [1.482], [4.5e+09], [3108064], [16019], [5.078],
  [rho-neg], [`W37a8388076b0`], [16], [verified], [2.235], [6.789e+09], [–], [–], [2.353],
  [ic-wied], [`W37a8388076b0`], [16], [verified], [1.49], [4.524e+09], [3124998], [15980], [4.931],
)

All values are recorded fields (`S` = `cost.s`, GAE = `cost.total_gae`, trials and relations from `counters`, wall = `time.solve_wall_ns` ÷ 3.6·10¹²); no run here is a replay. Records: `sessions/e-m16/records.jsonl`. Audit receipts: `audits/`.

#figure(image("runs.svg", width: 92%), caption: [Recorded `S` per run, session `ECBS1hd81d73afe25e`.])
