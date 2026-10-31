#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: itemF-control-run-20261018T2128Z

Descriptive record of one recorded run of examples/n2\_routes.rs, mode control (Amendment F) under
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
commit c51acec659201b5aed5a412ca673ea3ccabfe761
cmd: n2 research/weil_descent_angles_20261018/results/itemF-control-run-20261018T2128Z control (N2_THREADS=4, sizes 20..40)
rustc 1.97.0 (2d8144b78 2026-07-07)
4
Sun Oct 18 21:28:51 UTC 2026
post-run (d2edb2ab8): the tool was rustfmt-formatted (format only; unit tests pass); outputs unaffected.
```
== control\_summary.txt
```
F bits 20 Jind: runs 9 steps/p mean 0.2949 sd 0.1192 | repeated divisors per run 0.11 | solved by a repeat 1/9
F bits 20 Jwalk: runs 9 steps/p mean 0.2439 sd 0.1300 | repeated divisors per run 0.33 | solved by a repeat 3/9
F bits 24 Jind: runs 9 steps/p mean 0.3221 sd 0.1617 | repeated divisors per run 0.11 | solved by a repeat 1/9
F bits 24 Jwalk: runs 9 steps/p mean 0.3651 sd 0.1595 | repeated divisors per run 0.33 | solved by a repeat 3/9
F bits 28 Jind: runs 9 steps/p mean 0.3920 sd 0.0727 | repeated divisors per run 0.00 | solved by a repeat 0/9
F bits 28 Jwalk: runs 9 steps/p mean 0.3434 sd 0.1207 | repeated divisors per run 0.22 | solved by a repeat 2/9
F bits 32 Jind: runs 9 steps/p mean 0.2897 sd 0.1118 | repeated divisors per run 0.33 | solved by a repeat 3/9
F bits 32 Jwalk: runs 9 steps/p mean 0.2999 sd 0.1097 | repeated divisors per run 0.67 | solved by a repeat 6/9
F bits 36 Jind: runs 9 steps/p mean 0.3703 sd 0.0986 | repeated divisors per run 0.22 | solved by a repeat 2/9
F bits 36 Jwalk: runs 9 steps/p mean 0.3608 sd 0.1270 | repeated divisors per run 0.33 | solved by a repeat 3/9
F bits 40 Jind: runs 9 steps/p mean 0.3174 sd 0.0639 | repeated divisors per run 0.11 | solved by a repeat 1/9
F bits 40 Jwalk: runs 9 steps/p mean 0.3300 sd 0.1517 | repeated divisors per run 0.44 | solved by a repeat 4/9
F by cofactor Jind h=4: runs 33 steps/p mean 0.3688 sd 0.0981 (s.e. 0.0171)
F by cofactor Jind h=8: runs 21 steps/p mean 0.2718 sd 0.1058 (s.e. 0.0231)
F by cofactor Jwalk h=4: runs 33 steps/p mean 0.3443 sd 0.1316 (s.e. 0.0229)
F by cofactor Jwalk h=8: runs 21 steps/p mean 0.2917 sd 0.1358 (s.e. 0.0296)
F all answers verified: true
```
== control.csv (first 12 of 108 rows)
#table(columns: 12, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*bits*], [*instance*], [*p*], [*h*], [*ell*], [*target*], [*arm*], [*steps*], [*relations*], [*repeat\_events*], [*solved\_by\_repeat*], [*verified*],
[20], [0], [1091], [8], [148873], [0], [Jwalk], [265], [125], [1], [1], [1],
[20], [0], [1091], [8], [148873], [0], [Jind], [413], [206], [0], [0], [1],
[20], [0], [1091], [8], [148873], [1], [Jwalk], [550], [259], [0], [0], [1],
[20], [0], [1091], [8], [148873], [1], [Jind], [171], [82], [0], [0], [1],
[20], [0], [1091], [8], [148873], [2], [Jwalk], [107], [57], [0], [0], [1],
[20], [0], [1091], [8], [148873], [2], [Jind], [458], [213], [0], [0], [1],
[20], [1], [1061], [8], [140627], [0], [Jwalk], [243], [120], [1], [1], [1],
[20], [1], [1061], [8], [140627], [0], [Jind], [210], [109], [0], [0], [1],
[20], [1], [1061], [8], [140627], [1], [Jwalk], [45], [28], [0], [0], [1],
[20], [1], [1061], [8], [140627], [1], [Jind], [230], [108], [0], [0], [1],
[20], [1], [1061], [8], [140627], [2], [Jwalk], [308], [166], [0], [0], [1],
[20], [1], [1061], [8], [140627], [2], [Jind], [380], [207], [0], [0], [1]
)

== Graph
#figure(image("../../figures/itemF_control.svg", width: 100%), caption: [Amendment F: Arm J trials per p, walk vs independent trials, by cofactor h; each dot one verified run.])
