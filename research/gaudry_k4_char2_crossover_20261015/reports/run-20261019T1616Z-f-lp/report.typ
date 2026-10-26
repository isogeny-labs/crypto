#set page(paper: "a4", margin: 1.7cm)
#set text(size: 9.3pt)
#set table(stroke: 0.4pt)
#show table: set text(size: 8pt)
= Run report: ecbench session `ECBS1h1c3920570a3d` (f-lp), 2026-10-19T16:16Z

Part F, large primes, `m = 10, 12, 14`. Spec `specs/SPEC-F-lp.json`. Binary sha256 `c143cf62…`, commit `e403626cb` (clean). Host: Xeon \@ 2.10 GHz, 4 vCPU, one core per session (isolation level L1 or L0, recorded per run: wall time is descriptive only). The session completed: every planned run was recorded.

#table(columns: 9, align: (left, left, right, left, right, right, right, right, right),
  [arm], [workload], [$m$], [status], [$S$], [GAE], [trials], [relations], [wall (h)],
  [rho-neg], [`W845d6165060e`], [10], [verified], [0.6905], [5.119e+05], [–], [–], [0.000],
  [ic-wied], [`W845d6165060e`], [10], [verified], [66.89], [4.959e+07], [42623], [253], [0.037],
  [lp1-700], [`W845d6165060e`], [10], [verified], [107.1], [7.942e+07], [68334], [183], [0.059],
  [lp2-700], [`W845d6165060e`], [10], [verified], [75.26], [5.58e+07], [47974], [183], [0.039],
  [lp2-500], [`W845d6165060e`], [10], [verified], [101], [7.485e+07], [64397], [129], [0.053],
  [rho-neg], [`W739a83ada978`], [12], [verified], [1.017], [5.398e+06], [–], [–], [0.001],
  [ic-wied], [`W739a83ada978`], [12], [verified], [44.48], [2.36e+08], [188059], [1029], [0.159],
  [lp1-700], [`W739a83ada978`], [12], [verified], [65.83], [3.493e+08], [278445], [720], [0.241],
  [lp2-700], [`W739a83ada978`], [12], [verified], [48.1], [2.552e+08], [203380], [723], [0.173],
  [lp2-500], [`W739a83ada978`], [12], [verified], [63.12], [3.349e+08], [266982], [517], [0.228],
  [rho-neg], [`Wc066172da07a`], [14], [verified], [1.477], [2.803e+08], [–], [–], [0.049],
  [ic-wied], [`Wc066172da07a`], [14], [verified], [5.326], [1.011e+09], [749546], [4039], [0.662],
  [lp1-700], [`Wc066172da07a`], [14], [verified], [8.189], [1.554e+09], [1153920], [2848], [1.029],
  [lp2-700], [`Wc066172da07a`], [14], [verified], [5.83], [1.107e+09], [820985], [2832], [0.728],
  [lp2-500], [`Wc066172da07a`], [14], [verified], [7.773], [1.475e+09], [1095417], [2041], [0.961],
)

All values are recorded fields (`S` = `cost.s`, GAE = `cost.total_gae`, trials and relations from `counters`, wall = `time.solve_wall_ns` ÷ 3.6·10¹²); no run here is a replay. Records: `sessions/f-lp/records.jsonl`. Audit receipts: `audits/`.

#figure(image("runs.svg", width: 92%), caption: [Recorded `S` per run, session `ECBS1h1c3920570a3d`.])
