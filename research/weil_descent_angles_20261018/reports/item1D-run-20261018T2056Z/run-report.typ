#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: item1D-run-20261018T2056Z

Descriptive record of one recorded run of examples/decomposition\_coverage.rs (Part D) under
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
run: item1D, 20261018T2056Z
protocol commits: 55ebe41b6 (Addendum A), Amendment B (Poisson residual, frozen before this run); tool commit: fe0469c07
rustc 1.97.0 (2d8144b78 2026-07-07)
host: Linux 6.18.44-fc-v139 x86_64, 4 cores, Intel(R) Xeon(R) Processor @ 2.10GHz, 15 GB
command: rustc --edition 2021 -O examples/decomposition_coverage.rs -o tool && DC_THREADS=2 ./tool research/weil_descent_angles_20261018/results/item1D-run-20261018T2056Z
tool sha256: e4e38c882b23b23fc72750b47779e8113e7827f3a092a8d9b6d7ad3d406e0a73
threads: 2 (sharing the host with the item-4 census and the other coverage run; outputs are exact counts, no timing is evidence)
smoke runs before this run (not evidence): DC_SCALE=0.1 / PKM_SCALE=0.15, single thread.
post-run (d2edb2ab8): the tool was rustfmt-formatted (format only; unit tests pass); outputs unaffected.
```
== tool\_log.txt
```
jobs: 273
D char 0 n=2: curves 672 lambda 0.5033 | pi subfield 0.3949 null 0.3931 positive 0.0053 1/n! 0.5000 | resid subfield +0.00017 null +0.00001 diff +0.00016 (z 0.94)
D char 0 n=3: curves 63 lambda 0.1864 | pi subfield 0.1576 null 0.1787 positive 0.0029 1/n! 0.1667 | resid subfield -0.00985 null -0.00977 diff -0.00007 (z -0.10)
D char 0 n=4: curves 21 lambda 0.0831 | pi subfield 0.0605 null 0.0625 positive 0.0073 1/n! 0.0417 | resid subfield -0.01779 null -0.01691 diff -0.00088 (z -0.19)
D char 0 n=5: curves 9 lambda 0.0369 | pi subfield 0.0210 null 0.0187 positive 0.0032 1/n! 0.0083 | resid subfield -0.01439 null -0.01174 diff -0.00265 (z -0.43)
D char 2 n=2: curves 21 lambda 0.5073 | pi subfield 0.3730 null 0.4054 positive 0.0296 1/n! 0.5000 | resid subfield -0.02465 null +0.00615 diff -0.03080 (z -4.72)
D char 2 n=3: curves 15 lambda 0.1683 | pi subfield 0.1385 null 0.2039 positive 0.0065 1/n! 0.1667 | resid subfield -0.01150 null -0.02743 diff +0.01592 (z 1.96)
D char 2 n=4: curves 9 lambda 0.1467 | pi subfield 0.0818 null 0.0711 positive 0.0037 1/n! 0.0417 | resid subfield -0.03826 null -0.02154 diff -0.01672 (z -1.06)
D char 2 n=5: curves 9 lambda 0.0493 | pi subfield 0.0265 null 0.0529 positive 0.0057 1/n! 0.0083 | resid subfield -0.02066 null -0.04763 diff +0.02696 (z 1.11)
total 134.0s
```
== checks.txt
```
D char 0 n=2: curves 672 lambda 0.5033 | pi subfield 0.3949 null 0.3931 positive 0.0053 1/n! 0.5000 | resid subfield +0.00017 null +0.00001 diff +0.00016 (z 0.94)
D char 0 n=3: curves 63 lambda 0.1864 | pi subfield 0.1576 null 0.1787 positive 0.0029 1/n! 0.1667 | resid subfield -0.00985 null -0.00977 diff -0.00007 (z -0.10)
D char 0 n=4: curves 21 lambda 0.0831 | pi subfield 0.0605 null 0.0625 positive 0.0073 1/n! 0.0417 | resid subfield -0.01779 null -0.01691 diff -0.00088 (z -0.19)
D char 0 n=5: curves 9 lambda 0.0369 | pi subfield 0.0210 null 0.0187 positive 0.0032 1/n! 0.0083 | resid subfield -0.01439 null -0.01174 diff -0.00265 (z -0.43)
D char 2 n=2: curves 21 lambda 0.5073 | pi subfield 0.3730 null 0.4054 positive 0.0296 1/n! 0.5000 | resid subfield -0.02465 null +0.00615 diff -0.03080 (z -4.72)
D char 2 n=3: curves 15 lambda 0.1683 | pi subfield 0.1385 null 0.2039 positive 0.0065 1/n! 0.1667 | resid subfield -0.01150 null -0.02743 diff +0.01592 (z 1.96)
D char 2 n=4: curves 9 lambda 0.1467 | pi subfield 0.0818 null 0.0711 positive 0.0037 1/n! 0.0417 | resid subfield -0.03826 null -0.02154 diff -0.01672 (z -1.06)
D char 2 n=5: curves 9 lambda 0.0493 | pi subfield 0.0265 null 0.0529 positive 0.0057 1/n! 0.0083 | resid subfield -0.02066 null -0.04763 diff +0.02696 (z 1.11)
```
== coverage.csv (first 12 of 2457 rows)
#table(columns: 14, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*char*], [*n*], [*q*], [*curve*], [*order*], [*arm*], [*x\_values*], [*F*], [*lambda*], [*poisson*], [*inv\_n\_factorial*], [*distinct*], [*multisets*], [*pi*],
[0], [2], [53], [0], [2749], [subfield], [53], [56], [0.580575], [0.440423], [0.500000], [1221], [1596], [0.444162],
[0], [2], [53], [0], [2749], [null], [53], [64], [0.756639], [0.530759], [0.500000], [1411], [2080], [0.513278],
[0], [2], [53], [0], [2749], [positive], [53], [56], [0.580575], [0.440423], [0.500000], [113], [1596], [0.041106],
[0], [2], [53], [1], [2746], [subfield], [53], [50], [0.464312], [0.371432], [0.500000], [1007], [1275], [0.366715],
[0], [2], [53], [1], [2746], [null], [53], [54], [0.540787], [0.417710], [0.500000], [1129], [1485], [0.411143],
[0], [2], [53], [1], [2746], [positive], [53], [50], [0.464312], [0.371432], [0.500000], [101], [1275], [0.036781],
[0], [2], [53], [2], [2864], [subfield], [53], [52], [0.481145], [0.381925], [0.500000], [1121], [1378], [0.391411],
[0], [2], [53], [2], [2864], [null], [53], [36], [0.232542], [0.207483], [0.500000], [580], [666], [0.202514],
[0], [2], [53], [2], [2864], [positive], [53], [52], [0.481145], [0.381925], [0.500000], [105], [1378], [0.036662],
[0], [2], [59], [0], [3552], [subfield], [59], [54], [0.418074], [0.341687], [0.500000], [1208], [1485], [0.340090],
[0], [2], [59], [0], [3552], [null], [59], [72], [0.739865], [0.522822], [0.500000], [1888], [2628], [0.531532],
[0], [2], [59], [0], [3552], [positive], [59], [54], [0.418074], [0.341687], [0.500000], [109], [1485], [0.030687]
)
== coverage\_summary.csv (first 8 of 8 rows)
#table(columns: 13, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*char*], [*n*], [*curves*], [*mean\_lambda\_subfield*], [*mean\_pi\_subfield*], [*mean\_pi\_null*], [*mean\_pi\_positive*], [*inv\_n\_factorial*], [*resid\_subfield*], [*resid\_null*], [*resid\_diff*], [*sd\_resid\_diff*], [*z\_resid\_diff*],
[0], [2], [672], [0.503345], [0.394904], [0.393134], [0.005322], [0.500000], [0.000170], [0.000012], [0.000158], [0.004338], [0.94],
[0], [3], [63], [0.186410], [0.157583], [0.178683], [0.002853], [0.166667], [-0.009846], [-0.009773], [-0.000073], [0.005621], [-0.10],
[0], [4], [21], [0.083102], [0.060455], [0.062501], [0.007257], [0.041667], [-0.017793], [-0.016909], [-0.000883], [0.020966], [-0.19],
[0], [5], [9], [0.036891], [0.020982], [0.018669], [0.003245], [0.008333], [-0.014387], [-0.011739], [-0.002649], [0.018667], [-0.43],
[2], [2], [21], [0.507316], [0.372989], [0.405361], [0.029629], [0.500000], [-0.024652], [0.006151], [-0.030803], [0.029887], [-4.72],
[2], [3], [15], [0.168324], [0.138491], [0.203864], [0.006471], [0.166667], [-0.011502], [-0.027426], [0.015924], [0.031511], [1.96],
[2], [4], [9], [0.146729], [0.081766], [0.071056], [0.003683], [0.041667], [-0.038259], [-0.021542], [-0.016717], [0.047470], [-1.06],
[2], [5], [9], [0.049294], [0.026534], [0.052920], [0.005659], [0.008333], [-0.020663], [-0.047627], [0.026964], [0.073180], [1.11]
)

== Graph
#figure(image("../../figures/item1_coverage.svg", width: 100%), caption: [Part D: paired residual from Poisson per (characteristic, n), and binary n = 2 coverage per curve; exact.])
