#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: itemOJ-run-20261018T2132Z

Descriptive record of one recorded run of examples/n2\_routes.rs, mode run (Parts O/J) under
`research/weil_descent_angles_20261018/PROTOCOL.md` (Addendum A and
Amendments B–G, each committed before the run it governs). It lists what
ran, on what host, and what the tool recorded, file by file. Interpretation
is in the study's `REPORT.md`. Costs are counted operations
(F#sub[p] multiplications and inversions, group operations, native
operations), never wall-clock; the only times are the tool's own seconds.
Isolation: none earned (a shared cloud container, 4 threads); this
affects only the seconds, since every recorded value is an exact count or
a counted operation total.

== env.txt
```
commit e8568184d74fa862a5ea28e3a52285b249811958
cmd: n2 research/weil_descent_angles_20261018/results/itemOJ-run-20261018T2132Z run (N2_THREADS=4, sizes 20..40, 10 targets per instance)
rustc 1.97.0 (2d8144b78 2026-07-07)
4
Sun Oct 18 21:32:29 UTC 2026
post-run (d2edb2ab8): summary.txt, routes_summary.csv and ranking.csv re-derived from this run's routes.csv by `n2_routes <dir> summarize` (commit 570037637; adds the K6 lines); routes.csv unchanged. Tool then rustfmt-formatted (format only, tests pass).
```
== tool\_log.txt
```
jobs 18 total 166.7s
```
== summary.txt
```
O/J bits 20 arm J c 1: IC/rho ratio in [0.507, 1.529] over 12 cells -> unresolved; steps/p 0.311 (rho 0.327)
O/J bits 20 arm J c 2: IC/rho ratio in [0.510, 1.527] over 12 cells -> unresolved; steps/p 0.307 (rho 0.327)
O/J bits 20 arm J c 4: IC/rho ratio in [0.529, 1.556] over 12 cells -> unresolved; steps/p 0.322 (rho 0.327)
O/J bits 20 arm D c 1: IC/rho ratio in [2.796, 3.935] over 6 cells -> rho cheaper in every cell; steps/p 0.371 (rho 0.327)
O/J bits 20 arm D c 2: IC/rho ratio in [2.731, 3.833] over 6 cells -> rho cheaper in every cell; steps/p 0.358 (rho 0.327)
O/J bits 20 arm D c 4: IC/rho ratio in [3.067, 4.330] over 6 cells -> rho cheaper in every cell; steps/p 0.409 (rho 0.327)
O/J bits 24 arm J c 1: IC/rho ratio in [0.594, 1.729] over 12 cells -> unresolved; steps/p 0.307 (rho 0.346)
O/J bits 24 arm J c 2: IC/rho ratio in [0.640, 1.844] over 12 cells -> unresolved; steps/p 0.351 (rho 0.346)
O/J bits 24 arm J c 4: IC/rho ratio in [0.628, 1.783] over 12 cells -> unresolved; steps/p 0.321 (rho 0.346)
O/J bits 24 arm D c 1: IC/rho ratio in [8.414, 14.038] over 6 cells -> rho cheaper in every cell; steps/p 0.450 (rho 0.346)
O/J bits 24 arm D c 2: IC/rho ratio in [7.362, 12.245] over 6 cells -> rho cheaper in every cell; steps/p 0.391 (rho 0.346)
O/J bits 24 arm D c 4: IC/rho ratio in [8.035, 13.371] over 6 cells -> rho cheaper in every cell; steps/p 0.425 (rho 0.346)
O/J bits 28 arm J c 1: IC/rho ratio in [0.632, 1.825] over 12 cells -> unresolved; steps/p 0.294 (rho 0.429)
O/J bits 28 arm J c 2: IC/rho ratio in [0.730, 2.117] over 12 cells -> unresolved; steps/p 0.357 (rho 0.429)
O/J bits 28 arm J c 4: IC/rho ratio in [0.692, 2.001] over 12 cells -> unresolved; steps/p 0.328 (rho 0.429)
O/J bits 28 arm D c 1: IC/rho ratio in [12.289, 23.919] over 6 cells -> rho cheaper in every cell; steps/p 0.358 (rho 0.429)
O/J bits 28 arm D c 2: IC/rho ratio in [13.899, 27.019] over 6 cells -> rho cheaper in every cell; steps/p 0.409 (rho 0.429)
O/J bits 28 arm D c 4: IC/rho ratio in [12.354, 24.013] over 6 cells -> rho cheaper in every cell; steps/p 0.360 (rho 0.429)
O/J bits 32 arm J c 1: IC/rho ratio in [0.813, 2.386] over 12 cells -> unresolved; steps/p 0.298 (rho 0.361)
O/J bits 32 arm J c 2: IC/rho ratio in [0.843, 2.469] over 12 cells -> unresolved; steps/p 0.309 (rho 0.361)
O/J bits 32 arm J c 4: IC/rho ratio in [0.726, 2.119] over 12 cells -> unresolved; steps/p 0.259 (rho 0.361)
O/J bits 32 arm D c 1: IC/rho ratio in [18.276, 40.019] over 6 cells -> rho cheaper in every cell; steps/p 0.332 (rho 0.361)
O/J bits 32 arm D c 2: IC/rho ratio in [22.633, 49.542] over 6 cells -> rho cheaper in every cell; steps/p 0.412 (rho 0.361)
O/J bits 32 arm D c 4: IC/rho ratio in [17.812, 38.992] over 6 cells -> rho cheaper in every cell; steps/p 0.322 (rho 0.361)
O/J bits 36 arm J c 1: IC/rho ratio in [0.734, 2.179] over 12 cells -> unresolved; steps/p 0.342 (rho 0.498)
O/J bits 36 arm J c 2: IC/rho ratio in [0.746, 2.215] over 12 cells -> unresolved; steps/p 0.347 (rho 0.498)
O/J bits 36 arm J c 4: IC/rho ratio in [0.772, 2.299] over 12 cells -> unresolved; steps/p 0.360 (rho 0.498)
O/J bits 36 arm D c 1: IC/rho ratio in [18.403, 45.115] over 6 cells -> rho cheaper in every cell; steps/p 0.374 (rho 0.498)
O/J bits 36 arm D c 2: IC/rho ratio in [20.955, 51.332] over 6 cells -> rho cheaper in every cell; steps/p 0.425 (rho 0.498)
O/J bits 36 arm D c 4: IC/rho ratio in [19.179, 46.997] over 6 cells -> rho cheaper in every cell; steps/p 0.388 (rho 0.498)
O/J bits 40 arm J c 1: IC/rho ratio in [0.942, 2.822] over 12 cells -> unresolved; steps/p 0.337 (rho 0.388)
O/J bits 40 arm J c 2: IC/rho ratio in [0.945, 2.826] over 12 cells -> unresolved; steps/p 0.339 (rho 0.388)
O/J bits 40 arm J c 4: IC/rho ratio in [0.932, 2.787] over 12 cells -> unresolved; steps/p 0.335 (rho 0.388)
O/J bits 40 arm D c 1: IC/rho ratio in [25.118, 67.119] over 6 cells -> rho cheaper in every cell; steps/p 0.368 (rho 0.388)
O/J bits 40 arm D c 2: IC/rho ratio in [24.775, 66.327] over 6 cells -> rho cheaper in every cell; steps/p 0.359 (rho 0.388)
O/J bits 40 arm D c 4: IC/rho ratio in [26.269, 70.104] over 6 cells -> rho cheaper in every cell; steps/p 0.386 (rho 0.388)
O/J scaling arm R c 0: cost ~ p^0.696
O/J scaling arm J c 1: cost ~ p^0.779
O/J scaling arm J c 2: cost ~ p^0.775
O/J scaling arm J c 4: cost ~ p^0.774
O/J scaling arm D c 1: cost ~ p^1.068
O/J scaling arm D c 2: cost ~ p^1.088
O/J scaling arm D c 4: cost ~ p^1.068
K6 bits 20: rho steps/p mean 0.3270 (s.e. 0.0287, 30 runs) vs 0.886*sqrt(ell)/p mean 0.3566
K6 bits 24: rho steps/p mean 0.3458 (s.e. 0.0384, 30 runs) vs 0.886*sqrt(ell)/p mean 0.3997
K6 bits 28: rho steps/p mean 0.4286 (s.e. 0.0437, 30 runs) vs 0.886*sqrt(ell)/p mean 0.3998
K6 bits 32: rho steps/p mean 0.3610 (s.e. 0.0483, 30 runs) vs 0.886*sqrt(ell)/p mean 0.3565
K6 bits 36: rho steps/p mean 0.4985 (s.e. 0.0542, 30 runs) vs 0.886*sqrt(ell)/p mean 0.4430
K6 bits 40: rho steps/p mean 0.3882 (s.e. 0.0341, 30 runs) vs 0.886*sqrt(ell)/p mean 0.3997
O/J all answers verified: true
```
== ranking.csv (first 12 of 36 rows)
#table(columns: 9, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*bits*], [*arm*], [*c*], [*cells*], [*ic\_cheaper\_cells*], [*rho\_cheaper\_cells*], [*min\_ratio*], [*max\_ratio*], [*verdict*],
[20], [J], [1], [12], [6], [6], [0.5069], [1.5289], [unresolved],
[20], [J], [2], [12], [6], [6], [0.5102], [1.5267], [unresolved],
[20], [J], [4], [12], [6], [6], [0.5286], [1.5557], [unresolved],
[20], [D], [1], [6], [0], [6], [2.7965], [3.9349], [rho cheaper in every cell],
[20], [D], [2], [6], [0], [6], [2.7308], [3.8330], [rho cheaper in every cell],
[20], [D], [4], [6], [0], [6], [3.0671], [4.3299], [rho cheaper in every cell],
[24], [J], [1], [12], [6], [6], [0.5942], [1.7286], [unresolved],
[24], [J], [2], [12], [6], [6], [0.6405], [1.8440], [unresolved],
[24], [J], [4], [12], [6], [6], [0.6283], [1.7834], [unresolved],
[24], [D], [1], [6], [0], [6], [8.4135], [14.0384], [rho cheaper in every cell],
[24], [D], [2], [6], [0], [6], [7.3617], [12.2449], [rho cheaper in every cell],
[24], [D], [4], [6], [0], [6], [8.0353], [13.3714], [rho cheaper in every cell]
)
== routes.csv (first 12 of 1260 rows)
#table(columns: 26, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*bits*], [*instance*], [*p*], [*N*], [*h*], [*ell*], [*target*], [*arm*], [*c*], [*factor\_base*], [*steps*], [*relations*], [*b\_relations*], [*jac\_ops*], [*walk\_M*], [*walk\_I*], [*walk\_native*], [*jac\_M*], [*jac\_I*], [*la\_M*], [*la\_I*], [*la\_native*], [*eadd\_M*], [*eadd\_I*], [*verified*], [*seconds*],
[20], [0], [1091], [1190984], [8], [148873], [0], [R], [0], [0], [291], [0], [0], [0], [49105], [3270], [599], [0], [0], [0], [0], [0], [12], [1], [1], [0.001],
[20], [0], [1091], [1190984], [8], [148873], [0], [J], [1], [34], [281], [151], [1], [1941], [89746], [2048], [942], [87753], [2025], [0], [0], [1045], [12], [1], [1], [0.001],
[20], [0], [1091], [1190984], [8], [148873], [0], [D], [1], [34], [607], [212], [2], [0], [322779], [7494], [2928], [0], [0], [0], [0], [1594], [12], [1], [1], [0.003],
[20], [0], [1091], [1190984], [8], [148873], [0], [J], [2], [67], [184], [114], [4], [1755], [81811], [1860], [724], [79867], [1837], [0], [0], [688], [12], [1], [1], [0.001],
[20], [0], [1091], [1190984], [8], [148873], [0], [D], [2], [67], [381], [166], [6], [0], [214174], [5420], [2289], [0], [0], [0], [0], [1135], [12], [1], [1], [0.002],
[20], [0], [1091], [1190984], [8], [148873], [0], [J], [4], [133], [200], [104], [6], [1812], [84998], [1910], [875], [82228], [1887], [0], [0], [560], [12], [1], [1], [0.001],
[20], [0], [1091], [1190984], [8], [148873], [0], [D], [4], [133], [369], [149], [13], [0], [212847], [5377], [2372], [0], [0], [0], [0], [815], [12], [1], [1], [0.002],
[20], [0], [1091], [1190984], [8], [148873], [1], [R], [0], [0], [317], [0], [0], [0], [49931], [3318], [673], [0], [0], [0], [0], [0], [12], [1], [1], [0.001],
[20], [0], [1091], [1190984], [8], [148873], [1], [J], [1], [34], [356], [182], [3], [1948], [87715], [1934], [1146], [85497], [1911], [0], [0], [1264], [12], [1], [1], [0.001],
[20], [0], [1091], [1190984], [8], [148873], [1], [D], [1], [34], [259], [92], [1], [0], [151137], [4086], [1345], [0], [0], [0], [0], [552], [12], [1], [1], [0.001],
[20], [0], [1091], [1190984], [8], [148873], [1], [J], [2], [67], [244], [123], [2], [1804], [81555], [1787], [875], [79370], [1764], [0], [0], [752], [12], [1], [1], [0.000],
[20], [0], [1091], [1190984], [8], [148873], [1], [D], [2], [67], [251], [99], [3], [0], [150811], [4114], [1519], [0], [0], [0], [0], [622], [12], [1], [1], [0.002]
)
== routes\_summary.csv (first 12 of 324 rows)
#table(columns: 14, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*bits*], [*arm*], [*c*], [*ipr*], [*jac\_pricing*], [*native\_w*], [*runs*], [*verified*], [*mean\_p*], [*mean\_cost*], [*mean\_rho\_cost*], [*ratio\_to\_rho*], [*mean\_steps\_over\_p*], [*rho\_steps\_over\_p*],
[20], [J], [1], [3M], [measured], [0], [30], [1], [1081], [94476.1], [62588.3], [1.5095], [0.3114], [0.3270],
[20], [J], [1], [3M], [measured], [0.1], [30], [1], [1081], [94711.0], [62662.7], [1.5114], [0.3114], [0.3270],
[20], [J], [1], [3M], [measured], [1], [30], [1], [1081], [96825.1], [63331.8], [1.5289], [0.3114], [0.3270],
[20], [J], [1], [3M], [eadd], [0], [30], [1], [1081], [31726.8], [62588.3], [0.5069], [0.3114], [0.3270],
[20], [J], [1], [3M], [eadd], [0.1], [30], [1], [1081], [31961.7], [62662.7], [0.5101], [0.3114], [0.3270],
[20], [J], [1], [3M], [eadd], [1], [30], [1], [1081], [34075.7], [63331.8], [0.5381], [0.3114], [0.3270],
[20], [J], [1], [2log2p\_M], [measured], [0], [30], [1], [1081], [128073.1], [122390.9], [1.0464], [0.3114], [0.3270],
[20], [J], [1], [2log2p\_M], [measured], [0.1], [30], [1], [1081], [128308.0], [122465.2], [1.0477], [0.3114], [0.3270],
[20], [J], [1], [2log2p\_M], [measured], [1], [30], [1], [1081], [130422.1], [123134.3], [1.0592], [0.3114], [0.3270],
[20], [J], [1], [2log2p\_M], [eadd], [0], [30], [1], [1081], [65911.6], [122390.9], [0.5385], [0.3114], [0.3270],
[20], [J], [1], [2log2p\_M], [eadd], [0.1], [30], [1], [1081], [66146.5], [122465.2], [0.5401], [0.3114], [0.3270],
[20], [J], [1], [2log2p\_M], [eadd], [1], [30], [1], [1081], [68260.6], [123134.3], [0.5544], [0.3114], [0.3270]
)

== Graph
#figure(image("../../figures/itemOJ_routes.svg", width: 100%), caption: [Parts O/J: walk steps per p by arm (left) and the IC/rho cost-ratio band over pricing cells (right); 30 verified runs per arm and size.])
