#set page(paper: "a4", margin: 1.7cm)
#set text(size: 9.3pt)
#set table(stroke: 0.4pt)
#show table: set text(size: 8pt)
= Run report: ecbench session `ECBS1hfb0c75e0a81f` (session-rates), 2026-10-15T17:27Z

Pilot P3: capped rates at `m = 16, 17, 18` (exhausted by design). Spec `pilot/SPEC-pilot-rates.json`. Binary sha256 `c143cf62…`, commit `unrecorde`. Host: Xeon \@ 2.10 GHz, 4 vCPU, one core per session (isolation level L1 or L0, recorded per run: wall time is descriptive only). A pilot, run before PROTOCOL.md; no claim rests on it.

#table(columns: 9, align: (left, left, right, left, right, right, right, right, right),
  [arm], [workload], [$m$], [status], [$S$], [GAE], [trials], [relations], [wall (h)],
  [rho-neg], [`W27bf15e73a61`], [16], [exhausted], [1.482e-06], [4500], [–], [–], [0.000],
  [ic-wied], [`W27bf15e73a61`], [16], [exhausted], [0.001004], [3.051e+06], [2000], [12], [0.002],
  [rho-neg], [`W216de2c0b2b5`], [17], [exhausted], [3.708e-07], [4505], [–], [–], [0.000],
  [ic-wied], [`W216de2c0b2b5`], [17], [exhausted], [0.0002897], [3.519e+06], [2000], [12], [0.003],
  [rho-neg], [`W4bdb9a646cde`], [18], [exhausted], [9.288e-08], [4513], [–], [–], [0.000],
  [ic-wied], [`W4bdb9a646cde`], [18], [exhausted], [6.96e-05], [3.382e+06], [2000], [12], [0.003],
)

All values are recorded fields (`S` = `cost.s`, GAE = `cost.total_gae`, trials and relations from `counters`, wall = `time.solve_wall_ns` ÷ 3.6·10¹²); no run here is a replay. Records: `pilot/session-rates/records.jsonl`. Audit receipts: `audits/`.

#figure(image("runs.svg", width: 92%), caption: [Recorded `S` per run, session `ECBS1hfb0c75e0a81f`.])
