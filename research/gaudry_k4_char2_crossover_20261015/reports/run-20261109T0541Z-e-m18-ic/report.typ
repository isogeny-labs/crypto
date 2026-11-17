#set page(paper: "a4", margin: 1.7cm)
#set text(size: 9.3pt)
#set table(stroke: 0.4pt)
#show table: set text(size: 8pt)
= Run report: ecbench session `ECBS1ha37fa158c297` (e-m18-ic), 2026-11-09T05:41Z

Part E, `m = 18`, IC (checkpointing binary, resumed). Spec `specs/SPEC-E-m18-ic.json`. Binary sha256 `ad1dd91a…`, commit `e20167b70` (clean). Host: Xeon \@ 2.10 GHz, 4 vCPU, one core per session (isolation level L1 or L0, recorded per run: wall time is descriptive only). The session completed: every planned run was recorded.

#table(columns: 9, align: (left, left, right, left, right, right, right, right, right),
  [arm], [workload], [$m$], [status], [$S$], [GAE], [trials], [relations], [wall (h)],
  [ic-wied], [`We3b266046ebb`], [18], [verified], [0.4004], [1.946e+10], [12389345], [64278], [10.407],
)

All values are recorded fields (`S` = `cost.s`, GAE = `cost.total_gae`, trials and relations from `counters`, wall = `time.solve_wall_ns` ÷ 3.6·10¹²); no run here is a replay. Records: `sessions/e-m18-ic/records.jsonl`. Audit receipts: `audits/`.

#figure(image("runs.svg", width: 92%), caption: [Recorded `S` per run, session `ECBS1ha37fa158c297`.])
