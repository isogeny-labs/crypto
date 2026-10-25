#set page(paper: "a4", margin: 2cm)
#set text(font: "DejaVu Sans", size: 9.5pt)
#show raw: set text(size: 8.5pt)

= Run report: PKM Kummer tower, m = 3, N = 21 with msolve, run 20261018T0627Z

Study `research/pkm_m3_n21_msolve_20261018/`, pod `zdec0w9xwevehz` (8 vCPU, cgroup 32 GB, cap 27.2 GB, one thread), commit `6667c066`, msolve 0.6.5 (Ubuntu package, sha256 `c2722288…`). Observations only; the decision-rule reading is in `RESULTS.md`.

== Runs

#block(text(size: 8pt)[#table(
  columns: (auto, auto, auto, auto, 1fr, auto, auto, auto),
  align: (left, right, left, right, left, right, right, right),
  table.header([system], [N], [role], [wall], [outcome], [rounds], [max deg], [peak VmHWM]),
  [`m3_t5_x1289903598`], [15], [control, committed D = 7], [2,700 s], [finished, exit 0, 1,299 s, one-element Gröbner basis], [25], [7], [2.35 GB],
  [`m3_t7_x221026520`], [21], [target 0], [15,458 s], [wall stop inside the 6th degree-7 round], [25], [7], [15.33 GB],
  [`m3_t7_x704078485`], [21], [target 1], [7,728 s], [wall stop inside the 4th degree-7 round], [23], [7], [8.88 GB],
  [`m3_t8_x1227422598`], [24], [extra], [8,328 s], [wall stop inside the 5th degree-7 round], [24], [7], [13.03 GB],
)])

Rounds and degrees from msolve's `-v 2` round table; VmHWM sampled once a second from `/proc` (measured).

== Degree-7 rounds at N = 21, target 0

#block(text(size: 8pt)[#table(
  columns: 7,
  align: (right, right, right, right, right, right, right),
  table.header([round], [pairs], [matrix], [density], [new], [zero], [round s]),
  [1], [20,631], [98,282 × 101,435], [1.06 %], [2,354], [18,321], [154.36],
  [2], [29,876], [170,445 × 194,842], [1.22 %], [4,269], [25,607], [1,127.36],
  [3], [33,319], [203,174 × 228,589], [1.45 %], [5,421], [28,044], [2,498.79],
  [4], [28,015], [255,388 × 307,477], [1.45 %], [5,043], [23,096], [4,633.73],
  [5], [20,804], [323,437 × 431,738], [1.06 %], [5,502], [15,486], [5,812.61],
  [6], [7,038], [384,152 × 528,099], [0.75 %], [stopped], [], [],
)])

Every completed degree-7 round produced thousands of new basis elements; no run reached the end of degree 7 and none selected a degree-8 pair. The N = 15 control finished with maximum degree 7 and a one-element Gröbner basis, as item 3 recorded for the same system (committed D = 7).

#figure(
  image("msolve-degree.svg", width: 100%),
  caption: [Run 20261018T0627Z, pod `zdec0w9xwevehz`. Left: degree of the pairs selected in each msolve round against cumulative round time per system (symlog), stop marked. Right: peak VmHWM and wall per run against the 27.2 GB cap. Recorded values only (`plot.py`).],
)

== Missing

No N = 21 or N = 24 run finished, so no D is recorded for them; `m3_t8` target 1 was not run; the `f4_fp_tower` cell stays blocked on a 192 GB host. Raw records in `runs/` and `progress.txt`.
