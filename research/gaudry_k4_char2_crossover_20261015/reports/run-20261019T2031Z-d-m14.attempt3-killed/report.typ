#set page(paper: "a4", margin: 1.7cm)
#set text(size: 9.3pt)
#set table(stroke: 0.4pt)
#show table: set text(size: 8pt)
= Run report: ecbench session `ECBS1h6e149c0f160b` (d-m14.attempt3-killed), 2026-10-19T20:31Z

Part D, `m = 14` (killed after 2 of 3 runs). Spec `specs/SPEC-D-m14.json`. Binary sha256 `c143cf62…`, commit `e403626cb` (clean). Host: Xeon \@ 2.10 GHz, 4 vCPU, one core per session (isolation level L1 or L0, recorded per run: wall time is descriptive only). The session was killed by a container restart: 2 of 3 planned runs were recorded and the rest are missing. Its status is `running`, `ecbench verify` fails exactly its `session\_complete` and `record\_count` checks, and its records were not replayed.

#table(columns: 9, align: (left, left, right, left, right, right, right, right, right),
  [arm], [workload], [$m$], [status], [$S$], [GAE], [trials], [relations], [wall (h)],
  [rho-neg], [`Wc066172da07a`], [14], [verified], [0.6251], [1.186e+08], [–], [–], [0.020],
  [ic-dense], [`Wc066172da07a`], [14], [verified], [5.331], [1.012e+09], [743965], [4036], [0.641],
)

All values are recorded fields (`S` = `cost.s`, GAE = `cost.total_gae`, trials and relations from `counters`, wall = `time.solve_wall_ns` ÷ 3.6·10¹²); no run here is a replay. Records: `sessions/d-m14.attempt3-killed/records.jsonl`. Audit receipts: `audits/`.

#figure(image("runs.svg", width: 92%), caption: [Recorded `S` per run, session `ECBS1h6e149c0f160b`.])
