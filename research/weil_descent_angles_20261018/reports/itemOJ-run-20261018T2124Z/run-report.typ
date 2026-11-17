#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: itemOJ-run-20261018T2124Z

Descriptive record of one recorded run of examples/n2\_routes.rs, mode run (Parts O/J) under
`research/weil_descent_angles_20261018/PROTOCOL.md` (Addendum A and
Amendments B–G and Addendum H, each committed before the run it governs). It lists what
ran, on what host, and what the tool recorded, file by file. Interpretation
is in the study's `REPORT.md`. Costs are counted operations
(F#sub[p] multiplications and inversions, group operations, native
operations), never wall-clock; the only times are the tool's own seconds.
Isolation: none earned (a shared cloud container, or a RunPod CPU pod whose lscpu is in host.txt); this
affects only the seconds, since every recorded value is an exact count or
a counted operation total.

== env.txt
```
commit 4396b057a1a918da5c98bbbed0b10d5d04776a75
cmd: n2 research/weil_descent_angles_20261018/results/itemOJ-run-20261018T2124Z check; n2 research/weil_descent_angles_20261018/results/itemOJ-run-20261018T2124Z run (N2_THREADS=4, sizes 20..40)
rustc 1.97.0 (2d8144b78 2026-07-07)
4
Sun Oct 18 21:24:50 UTC 2026
post-run (d2edb2ab8): the tool was rustfmt-formatted (format only; unit tests pass); outputs unaffected.
```
== tool\_log.txt
```
jobs 18 total 57.6s
```
== checks.txt
```
K1 decomposition oracle vs brute force (p <= 101): instances 4, points 6068, mismatches 0
K2-K4 p=263 N=69544 h=8 ell=8693: #C(F_p)=264 (p+1=264) | ell·P=O true | tau valid true hom true nonzero true ell·tau(P)=0 true | N·D=0 true
K2-K4 p=271 N=73624 h=8 ell=9203: #C(F_p)=272 (p+1=272) | ell·P=O true | tau valid true hom true nonzero true ell·tau(P)=0 true | N·D=0 true
K2-K4 p=1097 N=1205332 h=4 ell=301333: #C(F_p)=1098 (p+1=1098) | ell·P=O true | tau valid true hom true nonzero true ell·tau(P)=0 true | N·D=0 true
K2-K4 p=1069 N=1142056 h=8 ell=142757: #C(F_p)=1070 (p+1=1070) | ell·P=O true | tau valid true hom true nonzero true ell·tau(P)=0 true | N·D=0 true
K2-K4 p=4229 N=17878244 h=4 ell=4469561: #C(F_p)=4230 (p+1=4230) | ell·P=O true | tau valid true hom true nonzero true ell·tau(P)=0 true | N·D=0 true
K2-K4 p=4507 N=20306164 h=4 ell=5076541: #C(F_p)=4508 (p+1=4508) | ell·P=O true | tau valid true hom true nonzero true ell·tau(P)=0 true | N·D=0 true
K5 p=1123: genus-2 add fast 41.3M+1.01I cantor 112.8M+4.00I | double fast 48.0M+1.00I cantor 115.9M+3.99I | agree true | E(F_p^2) add 12M+1I
K5 p=4561: genus-2 add fast 41.0M+1.00I cantor 113.0M+4.00I | double fast 48.0M+1.00I cantor 116.0M+4.00I | agree true | E(F_p^2) add 12M+1I
K5 p=18251: genus-2 add fast 41.0M+1.00I cantor 113.0M+4.00I | double fast 48.0M+1.00I cantor 116.0M+4.00I | agree true | E(F_p^2) add 12M+1I
K5 p=71963: genus-2 add fast 41.0M+1.00I cantor 113.0M+4.00I | double fast 48.0M+1.00I cantor 116.0M+4.00I | agree true | E(F_p^2) add 12M+1I
```
== summary.txt
```
O/J bits 20 arm J c 1: IC/rho ratio in [0.507, 1.542] over 12 cells -> unresolved; steps/p 0.311 (rho 0.338)
O/J bits 20 arm J c 2: IC/rho ratio in [0.508, 1.533] over 12 cells -> unresolved; steps/p 0.311 (rho 0.338)
O/J bits 20 arm J c 4: IC/rho ratio in [0.501, 1.488] over 12 cells -> unresolved; steps/p 0.241 (rho 0.338)
O/J bits 20 arm D c 1: IC/rho ratio in [2.950, 4.183] over 6 cells -> rho cheaper in every cell; steps/p 0.397 (rho 0.338)
O/J bits 20 arm D c 2: IC/rho ratio in [2.489, 3.491] over 6 cells -> rho cheaper in every cell; steps/p 0.320 (rho 0.338)
O/J bits 20 arm D c 4: IC/rho ratio in [3.368, 4.806] over 6 cells -> rho cheaper in every cell; steps/p 0.457 (rho 0.338)
O/J bits 24 arm J c 1: IC/rho ratio in [0.661, 1.954] over 12 cells -> unresolved; steps/p 0.351 (rho 0.283)
O/J bits 24 arm J c 2: IC/rho ratio in [0.680, 1.995] over 12 cells -> unresolved; steps/p 0.364 (rho 0.283)
O/J bits 24 arm J c 4: IC/rho ratio in [0.659, 1.899] over 12 cells -> unresolved; steps/p 0.321 (rho 0.283)
O/J bits 24 arm D c 1: IC/rho ratio in [9.111, 15.079] over 6 cells -> rho cheaper in every cell; steps/p 0.461 (rho 0.283)
O/J bits 24 arm D c 2: IC/rho ratio in [9.781, 16.216] over 6 cells -> rho cheaper in every cell; steps/p 0.498 (rho 0.283)
O/J bits 24 arm D c 4: IC/rho ratio in [6.910, 11.350] over 6 cells -> rho cheaper in every cell; steps/p 0.339 (rho 0.283)
O/J bits 28 arm J c 1: IC/rho ratio in [0.707, 2.119] over 12 cells -> unresolved; steps/p 0.338 (rho 0.425)
O/J bits 28 arm J c 2: IC/rho ratio in [0.681, 2.036] over 12 cells -> unresolved; steps/p 0.320 (rho 0.425)
O/J bits 28 arm J c 4: IC/rho ratio in [0.700, 2.095] over 12 cells -> unresolved; steps/p 0.330 (rho 0.425)
O/J bits 28 arm D c 1: IC/rho ratio in [12.384, 23.683] over 6 cells -> rho cheaper in every cell; steps/p 0.357 (rho 0.425)
O/J bits 28 arm D c 2: IC/rho ratio in [13.013, 24.874] over 6 cells -> rho cheaper in every cell; steps/p 0.376 (rho 0.425)
O/J bits 28 arm D c 4: IC/rho ratio in [13.429, 25.760] over 6 cells -> rho cheaper in every cell; steps/p 0.379 (rho 0.425)
O/J bits 32 arm J c 1: IC/rho ratio in [0.900, 2.774] over 12 cells -> unresolved; steps/p 0.348 (rho 0.378)
O/J bits 32 arm J c 2: IC/rho ratio in [0.701, 2.143] over 12 cells -> unresolved; steps/p 0.262 (rho 0.378)
O/J bits 32 arm J c 4: IC/rho ratio in [0.888, 2.728] over 12 cells -> unresolved; steps/p 0.339 (rho 0.378)
O/J bits 32 arm D c 1: IC/rho ratio in [17.961, 38.797] over 6 cells -> rho cheaper in every cell; steps/p 0.331 (rho 0.378)
O/J bits 32 arm D c 2: IC/rho ratio in [20.006, 43.090] over 6 cells -> rho cheaper in every cell; steps/p 0.371 (rho 0.378)
O/J bits 32 arm D c 4: IC/rho ratio in [20.953, 45.133] over 6 cells -> rho cheaper in every cell; steps/p 0.389 (rho 0.378)
O/J bits 36 arm J c 1: IC/rho ratio in [0.589, 1.852] over 12 cells -> unresolved; steps/p 0.343 (rho 0.626)
O/J bits 36 arm J c 2: IC/rho ratio in [0.596, 1.874] over 12 cells -> unresolved; steps/p 0.347 (rho 0.626)
O/J bits 36 arm J c 4: IC/rho ratio in [0.566, 1.778] over 12 cells -> unresolved; steps/p 0.328 (rho 0.626)
O/J bits 36 arm D c 1: IC/rho ratio in [16.647, 40.044] over 6 cells -> rho cheaper in every cell; steps/p 0.418 (rho 0.626)
O/J bits 36 arm D c 2: IC/rho ratio in [16.535, 39.834] over 6 cells -> rho cheaper in every cell; steps/p 0.411 (rho 0.626)
O/J bits 36 arm D c 4: IC/rho ratio in [15.745, 37.966] over 6 cells -> rho cheaper in every cell; steps/p 0.393 (rho 0.626)
O/J bits 40 arm J c 1: IC/rho ratio in [0.528, 1.680] over 12 cells -> unresolved; steps/p 0.394 (rho 0.812)
O/J bits 40 arm J c 2: IC/rho ratio in [0.410, 1.301] over 12 cells -> unresolved; steps/p 0.305 (rho 0.812)
O/J bits 40 arm J c 4: IC/rho ratio in [0.370, 1.169] over 12 cells -> unresolved; steps/p 0.274 (rho 0.812)
O/J bits 40 arm D c 1: IC/rho ratio in [12.866, 33.867] over 6 cells -> rho cheaper in every cell; steps/p 0.379 (rho 0.812)
O/J bits 40 arm D c 2: IC/rho ratio in [12.720, 33.332] over 6 cells -> rho cheaper in every cell; steps/p 0.381 (rho 0.812)
O/J bits 40 arm D c 4: IC/rho ratio in [13.185, 34.676] over 6 cells -> rho cheaper in every cell; steps/p 0.391 (rho 0.812)
O/J scaling arm R c 0: cost ~ p^0.787
O/J scaling arm J c 1: cost ~ p^0.792
O/J scaling arm J c 2: cost ~ p^0.762
O/J scaling arm J c 4: cost ~ p^0.758
O/J scaling arm D c 1: cost ~ p^1.072
O/J scaling arm D c 2: cost ~ p^1.086
O/J scaling arm D c 4: cost ~ p^1.076
O/J all answers verified: true
```
== ranking.csv (first 12 of 36 rows)
#table(columns: 9, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*bits*], [*arm*], [*c*], [*cells*], [*ic\_cheaper\_cells*], [*rho\_cheaper\_cells*], [*min\_ratio*], [*max\_ratio*], [*verdict*],
[20], [J], [1], [12], [6], [6], [0.5069], [1.5417], [unresolved],
[20], [J], [2], [12], [6], [6], [0.5077], [1.5327], [unresolved],
[20], [J], [4], [12], [6], [6], [0.5007], [1.4878], [unresolved],
[20], [D], [1], [6], [0], [6], [2.9501], [4.1828], [rho cheaper in every cell],
[20], [D], [2], [6], [0], [6], [2.4895], [3.4906], [rho cheaper in every cell],
[20], [D], [4], [6], [0], [6], [3.3680], [4.8059], [rho cheaper in every cell],
[24], [J], [1], [12], [6], [6], [0.6609], [1.9539], [unresolved],
[24], [J], [2], [12], [6], [6], [0.6803], [1.9946], [unresolved],
[24], [J], [4], [12], [6], [6], [0.6595], [1.8993], [unresolved],
[24], [D], [1], [6], [0], [6], [9.1107], [15.0789], [rho cheaper in every cell],
[24], [D], [2], [6], [0], [6], [9.7812], [16.2163], [rho cheaper in every cell],
[24], [D], [4], [6], [0], [6], [6.9102], [11.3495], [rho cheaper in every cell]
)
== routes.csv (first 12 of 378 rows)
#table(columns: 26, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*bits*], [*instance*], [*p*], [*N*], [*h*], [*ell*], [*target*], [*arm*], [*c*], [*factor\_base*], [*steps*], [*relations*], [*b\_relations*], [*jac\_ops*], [*walk\_M*], [*walk\_I*], [*walk\_native*], [*jac\_M*], [*jac\_I*], [*la\_M*], [*la\_I*], [*la\_native*], [*eadd\_M*], [*eadd\_I*], [*verified*], [*seconds*],
[20], [0], [1091], [1190984], [8], [148873], [0], [R], [0], [0], [291], [0], [0], [0], [49105], [3270], [312], [0], [0], [0], [0], [0], [12], [1], [1], [0.001],
[20], [0], [1091], [1190984], [8], [148873], [0], [J], [1], [34], [281], [151], [1], [1941], [89746], [2048], [942], [87753], [2025], [0], [0], [1045], [12], [1], [1], [0.001],
[20], [0], [1091], [1190984], [8], [148873], [0], [D], [1], [34], [607], [212], [2], [0], [322779], [7494], [2928], [0], [0], [0], [0], [1594], [12], [1], [1], [0.003],
[20], [0], [1091], [1190984], [8], [148873], [0], [J], [2], [67], [184], [114], [4], [1755], [81811], [1860], [724], [79867], [1837], [0], [0], [688], [12], [1], [1], [0.001],
[20], [0], [1091], [1190984], [8], [148873], [0], [D], [2], [67], [381], [166], [6], [0], [214174], [5420], [2289], [0], [0], [0], [0], [1135], [12], [1], [1], [0.002],
[20], [0], [1091], [1190984], [8], [148873], [0], [J], [4], [133], [200], [104], [6], [1812], [84998], [1910], [875], [82228], [1887], [0], [0], [560], [12], [1], [1], [0.001],
[20], [0], [1091], [1190984], [8], [148873], [0], [D], [4], [133], [369], [149], [13], [0], [212847], [5377], [2372], [0], [0], [0], [0], [815], [12], [1], [1], [0.003],
[20], [0], [1091], [1190984], [8], [148873], [1], [R], [0], [0], [317], [0], [0], [0], [49931], [3318], [358], [0], [0], [0], [0], [0], [12], [1], [1], [0.001],
[20], [0], [1091], [1190984], [8], [148873], [1], [J], [1], [34], [356], [182], [3], [1948], [87715], [1934], [1146], [85497], [1911], [0], [0], [1264], [12], [1], [1], [0.001],
[20], [0], [1091], [1190984], [8], [148873], [1], [D], [1], [34], [259], [92], [1], [0], [151137], [4086], [1345], [0], [0], [0], [0], [552], [12], [1], [1], [0.001],
[20], [0], [1091], [1190984], [8], [148873], [1], [J], [2], [67], [244], [123], [2], [1804], [81555], [1787], [875], [79370], [1764], [0], [0], [752], [12], [1], [1], [0.001],
[20], [0], [1091], [1190984], [8], [148873], [1], [D], [2], [67], [251], [99], [3], [0], [150811], [4114], [1519], [0], [0], [0], [0], [622], [12], [1], [1], [0.001]
)
== routes\_summary.csv (first 12 of 324 rows)
#table(columns: 14, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*bits*], [*arm*], [*c*], [*ipr*], [*jac\_pricing*], [*native\_w*], [*runs*], [*verified*], [*mean\_p*], [*mean\_cost*], [*mean\_rho\_cost*], [*ratio\_to\_rho*], [*mean\_steps\_over\_p*], [*rho\_steps\_over\_p*],
[20], [J], [1], [3M], [measured], [0], [9], [1], [1081], [94779.2], [62581.1], [1.5145], [0.3112], [0.3379],
[20], [J], [1], [3M], [measured], [0.1], [9], [1], [1081], [95011.3], [62621.4], [1.5172], [0.3112], [0.3379],
[20], [J], [1], [3M], [measured], [1], [9], [1], [1081], [97099.9], [62984.0], [1.5417], [0.3112], [0.3379],
[20], [J], [1], [3M], [eadd], [0], [9], [1], [1081], [31725.0], [62581.1], [0.5069], [0.3112], [0.3379],
[20], [J], [1], [3M], [eadd], [0.1], [9], [1], [1081], [31957.1], [62621.4], [0.5103], [0.3112], [0.3379],
[20], [J], [1], [3M], [eadd], [1], [9], [1], [1081], [34045.7], [62984.0], [0.5405], [0.3112], [0.3379],
[20], [J], [1], [2log2p\_M], [measured], [0], [9], [1], [1081], [128568.6], [122380.8], [1.0506], [0.3112], [0.3379],
[20], [J], [1], [2log2p\_M], [measured], [0.1], [9], [1], [1081], [128800.7], [122421.1], [1.0521], [0.3112], [0.3379],
[20], [J], [1], [2log2p\_M], [measured], [1], [9], [1], [1081], [130889.3], [122783.7], [1.0660], [0.3112], [0.3379],
[20], [J], [1], [2log2p\_M], [eadd], [0], [9], [1], [1081], [65908.6], [122380.8], [0.5386], [0.3112], [0.3379],
[20], [J], [1], [2log2p\_M], [eadd], [0.1], [9], [1], [1081], [66140.7], [122421.1], [0.5403], [0.3112], [0.3379],
[20], [J], [1], [2log2p\_M], [eadd], [1], [9], [1], [1081], [68229.3], [122783.7], [0.5557], [0.3112], [0.3379]
)

== Graph
#figure(image("../../figures/itemOJ_routes.svg", width: 100%), caption: [Graph drawn from the recorded re-run 20261018T2132Z (this run's rho rows are superseded by Amendment G); steps/p and pricing bands.])
