#set page(paper: "a4", margin: 1.7cm)
#set text(size: 9.3pt)
#set table(stroke: 0.4pt)
#show table: set text(size: 8pt)
= Run report: ecbench session `ECBS1h636116972aca` (d-m14), 2026-11-09T05:41Z

Part D, `m = 14` (checkpointing binary). Spec `specs/SPEC-D-m14.json`. Binary sha256 `ad1dd91a…`, commit `e20167b70` (clean). Host: Xeon \@ 2.10 GHz, 4 vCPU, one core per session (isolation level L1 or L0, recorded per run: wall time is descriptive only). The session completed: every planned run was recorded.

#table(columns: 9, align: (left, left, right, left, right, right, right, right, right),
  [arm], [workload], [$m$], [status], [$S$], [GAE], [trials], [relations], [wall (h)],
  [rho-neg], [`Wc066172da07a`], [14], [verified], [0.6251], [1.186e+08], [–], [–], [0.020],
  [ic-dense], [`Wc066172da07a`], [14], [verified], [5.331], [1.012e+09], [743965], [4036], [0.609],
  [ic-wied], [`Wc066172da07a`], [14], [verified], [5.29], [1.004e+09], [744441], [4044], [0.632],
)

All values are recorded fields (`S` = `cost.s`, GAE = `cost.total_gae`, trials and relations from `counters`, wall = `time.solve_wall_ns` ÷ 3.6·10¹²); no run here is a replay. Records: `sessions/d-m14/records.jsonl`. Audit receipts: `audits/`.

#figure(image("runs.svg", width: 92%), caption: [Recorded `S` per run, session `ECBS1h636116972aca`.])
