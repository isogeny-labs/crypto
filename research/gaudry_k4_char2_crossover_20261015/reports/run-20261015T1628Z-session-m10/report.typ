#set page(paper: "a4", margin: 1.7cm)
#set text(size: 9.3pt)
#set table(stroke: 0.4pt)
#show table: set text(size: 8pt)
= Run report: ecbench session `ECBS1hcc349c199c2c` (session-m10), 2026-10-15T16:28Z

Pilot P2, `m = 10`: dense, `wiedemann`, large-prime settings. Spec `pilot/SPEC-pilot-m10.json`. Binary sha256 `50f35402…`, commit `e6375d605`. Host: Xeon \@ 2.10 GHz, 4 vCPU, one core per session (isolation level L1 or L0, recorded per run: wall time is descriptive only). A pilot, run before PROTOCOL.md; no claim rests on it.

#table(columns: 9, align: (left, left, right, left, right, right, right, right, right),
  [arm], [workload], [$m$], [status], [$S$], [GAE], [trials], [relations], [wall (h)],
  [rho-neg], [`W845d6165060e`], [10], [verified], [0.7222], [5.354e+05], [–], [–], [0.000],
  [ic-dense], [`W845d6165060e`], [10], [verified], [71.43], [5.296e+07], [45525], [255], [0.046],
  [ic-wied], [`W845d6165060e`], [10], [verified], [72.14], [5.349e+07], [45974], [256], [0.046],
  [lp1-500], [`W845d6165060e`], [10], [verified], [217], [1.609e+08], [138570], [129], [0.142],
  [lp2-300], [`W845d6165060e`], [10], [verified], [191.7], [1.421e+08], [122370], [79], [0.126],
  [lp2-500], [`W845d6165060e`], [10], [verified], [106.9], [7.927e+07], [68206], [129], [0.069],
  [lp2-700], [`W845d6165060e`], [10], [verified], [76.4], [5.664e+07], [48696], [180], [0.050],
  [rho-neg], [`Wc69dfe4568cc`], [10], [verified], [1.911], [1.417e+06], [–], [–], [0.000],
  [ic-dense], [`Wc69dfe4568cc`], [10], [verified], [73.42], [5.444e+07], [46810], [256], [0.049],
  [ic-wied], [`Wc69dfe4568cc`], [10], [verified], [73.84], [5.475e+07], [47074], [259], [0.050],
  [lp1-500], [`Wc69dfe4568cc`], [10], [verified], [198.3], [1.471e+08], [126626], [130], [0.128],
  [lp2-300], [`Wc69dfe4568cc`], [10], [verified], [193.3], [1.433e+08], [123404], [79], [0.125],
  [lp2-500], [`Wc69dfe4568cc`], [10], [verified], [97.98], [7.265e+07], [62497], [130], [0.062],
  [lp2-700], [`Wc69dfe4568cc`], [10], [verified], [79.21], [5.873e+07], [50504], [183], [0.050],
)

All values are recorded fields (`S` = `cost.s`, GAE = `cost.total_gae`, trials and relations from `counters`, wall = `time.solve_wall_ns` ÷ 3.6·10¹²); no run here is a replay. Records: `pilot/session-m10/records.jsonl`. Audit receipts: `audits/`.

#figure(image("runs.svg", width: 92%), caption: [Recorded `S` per run, session `ECBS1hcc349c199c2c`.])
