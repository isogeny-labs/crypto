#set page(paper: "a4", margin: 1.7cm)
#set text(size: 9.3pt)
#set table(stroke: 0.4pt)
#show table: set text(size: 8pt)
= Run report: ecbench session `ECBS1hcbee1f16c529` (d-ladder), 2026-10-19T19:20Z

Part D, sparse against dense, `m = 8, 10, 12`. Spec `specs/SPEC-D-ladder.json`. Binary sha256 `c143cf62…`, commit `e403626cb` (clean). Host: Xeon \@ 2.10 GHz, 4 vCPU, one core per session (isolation level L1 or L0, recorded per run: wall time is descriptive only). The session completed: every planned run was recorded.

#table(columns: 9, align: (left, left, right, left, right, right, right, right, right),
  [arm], [workload], [$m$], [status], [$S$], [GAE], [trials], [relations], [wall (h)],
  [rho-neg], [`Wf87c59c61bdd`], [8], [verified], [0.911], [2.438e+04], [–], [–], [0.000],
  [ic-dense], [`Wf87c59c61bdd`], [8], [verified], [396.9], [1.062e+07], [9861], [61], [0.008],
  [ic-wied], [`Wf87c59c61bdd`], [8], [verified], [396.9], [1.062e+07], [9861], [61], [0.008],
  [rho-neg], [`Wa195d33f5d8a`], [8], [verified], [2.181], [5.836e+04], [–], [–], [0.000],
  [ic-dense], [`Wa195d33f5d8a`], [8], [verified], [599.5], [1.604e+07], [14961], [62], [0.012],
  [ic-wied], [`Wa195d33f5d8a`], [8], [verified], [599.5], [1.604e+07], [14961], [62], [0.012],
  [rho-neg], [`W845d6165060e`], [10], [verified], [0.684], [5.071e+05], [–], [–], [0.000],
  [ic-dense], [`W845d6165060e`], [10], [verified], [72.99], [5.412e+07], [46525], [261], [0.040],
  [ic-wied], [`W845d6165060e`], [10], [verified], [73], [5.412e+07], [46525], [261], [0.038],
  [rho-neg], [`Wc69dfe4568cc`], [10], [verified], [1.88], [1.394e+06], [–], [–], [0.000],
  [ic-dense], [`Wc69dfe4568cc`], [10], [verified], [72.27], [5.359e+07], [46069], [258], [0.040],
  [ic-wied], [`Wc69dfe4568cc`], [10], [verified], [72.28], [5.359e+07], [46069], [258], [0.039],
  [rho-neg], [`W739a83ada978`], [12], [verified], [0.7947], [4.216e+06], [–], [–], [0.001],
  [ic-dense], [`W739a83ada978`], [12], [verified], [42.56], [2.258e+08], [179877], [1020], [0.153],
  [ic-wied], [`W739a83ada978`], [12], [verified], [42.72], [2.266e+08], [180596], [1024], [0.152],
  [rho-neg], [`W255399ed9512`], [12], [verified], [1.304], [6.918e+06], [–], [–], [0.001],
  [ic-dense], [`W255399ed9512`], [12], [verified], [45.01], [2.388e+08], [190242], [1025], [0.163],
  [ic-wied], [`W255399ed9512`], [12], [verified], [45.03], [2.389e+08], [190396], [1026], [0.163],
)

All values are recorded fields (`S` = `cost.s`, GAE = `cost.total_gae`, trials and relations from `counters`, wall = `time.solve_wall_ns` ÷ 3.6·10¹²); no run here is a replay. Records: `sessions/d-ladder/records.jsonl`. Audit receipts: `audits/`.

#figure(image("runs.svg", width: 92%), caption: [Recorded `S` per run, session `ECBS1hcbee1f16c529`.])
