#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: H5-aut-census-run-20261018T2227Z

Descriptive record of one recorded run of examples/genus2\_aut\_census.rs (Addendum H5) under
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
commit 0bd2a0f5c0b0f0005d57850c12466fa82b414c59 + examples/genus2_aut_census.rs (uncommitted at run time; committed with these results)
sha256 f86392542cd7b1f0203cd287b72d62b52c69131644c8d146db666832bdddf632
cmd: genus2_aut_census <dir> 5 7 11 13
rustc 1.97.0 (2d8144b78 2026-07-07)
4
Sun Oct 18 22:27:02 UTC 2026
```
== summary.txt
```
H5 p=5: genus-2 mass sum 1/|Aut| = 125.0000 (expected p^3 = 125)
H5 p=5: a1=0 SIMPLE over F_p (Res-shaped) |Aut|=2: 31 classes
H5 p=5: a1=0 SIMPLE over F_p (Res-shaped) |Aut|=4: 1 classes
H5 p=5: a1=0 SIMPLE over F_p (Res-shaped) |Aut|=6: 4 classes
H5 p=5: a1=0 SIMPLE over F_p (Res-shaped) |Aut|=120: 1 classes
H5 p=5: a1=0 split over F_p |Aut|=2: 7 classes
H5 p=5: a1=0 split over F_p |Aut|=4: 7 classes
H5 p=5: a1=0 split over F_p |Aut|=8: 1 classes
H5 p=5: a1=0 split over F_p |Aut|=12: 1 classes
H5 p=5: simple Res-shaped classes with |Aut| > 2: 6
H5 p=7: genus-2 mass sum 1/|Aut| = 343.0000 (expected p^3 = 343)
H5 p=7: a1=0 SIMPLE over F_p (Res-shaped) |Aut|=2: 85 classes
H5 p=7: a1=0 SIMPLE over F_p (Res-shaped) |Aut|=4: 3 classes
H5 p=7: a1=0 SIMPLE over F_p (Res-shaped) |Aut|=6: 2 classes
H5 p=7: a1=0 SIMPLE over F_p (Res-shaped) |Aut|=24: 1 classes
H5 p=7: a1=0 split over F_p |Aut|=2: 16 classes
H5 p=7: a1=0 split over F_p |Aut|=4: 14 classes
H5 p=7: a1=0 split over F_p |Aut|=6: 2 classes
H5 p=7: a1=0 split over F_p |Aut|=8: 1 classes
H5 p=7: a1=0 split over F_p |Aut|=12: 1 classes
H5 p=7: simple Res-shaped classes with |Aut| > 2: 6
H5 p=11: genus-2 mass sum 1/|Aut| = 1331.0000 (expected p^3 = 1331)
H5 p=11: a1=0 SIMPLE over F_p (Res-shaped) |Aut|=2: 306 classes
H5 p=11: a1=0 SIMPLE over F_p (Res-shaped) |Aut|=4: 3 classes
H5 p=11: a1=0 SIMPLE over F_p (Res-shaped) |Aut|=6: 9 classes
H5 p=11: a1=0 SIMPLE over F_p (Res-shaped) |Aut|=8: 1 classes
H5 p=11: a1=0 SIMPLE over F_p (Res-shaped) |Aut|=12: 1 classes
H5 p=11: a1=0 split over F_p |Aut|=2: 38 classes
H5 p=11: a1=0 split over F_p |Aut|=4: 30 classes
H5 p=11: a1=0 split over F_p |Aut|=8: 2 classes
H5 p=11: a1=0 split over F_p |Aut|=12: 3 classes
H5 p=11: simple Res-shaped classes with |Aut| > 2: 14
H5 p=13: genus-2 mass sum 1/|Aut| = 2197.0000 (expected p^3 = 2197)
H5 p=13: a1=0 SIMPLE over F_p (Res-shaped) |Aut|=2: 450 classes
H5 p=13: a1=0 SIMPLE over F_p (Res-shaped) |Aut|=4: 6 classes
H5 p=13: a1=0 SIMPLE over F_p (Res-shaped) |Aut|=6: 7 classes
H5 p=13: a1=0 SIMPLE over F_p (Res-shaped) |Aut|=24: 1 classes
H5 p=13: a1=0 split over F_p |Aut|=2: 97 classes
H5 p=13: a1=0 split over F_p |Aut|=4: 46 classes
H5 p=13: a1=0 split over F_p |Aut|=6: 2 classes
H5 p=13: a1=0 split over F_p |Aut|=8: 1 classes
H5 p=13: a1=0 split over F_p |Aut|=12: 1 classes
H5 p=13: simple Res-shaped classes with |Aut| > 2: 14
```
== classes.csv (first 12 of 8436 rows)
#table(columns: 11, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*p*], [*rep*], [*orbit*], [*aut*], [*n1*], [*n2*], [*a1*], [*a2*], [*even*], [*split*], [*trace\_E*],
[5], [0 1 0 0 0 1 0], [60], [8], [6], [46], [0], [10], [true], [true], [-10],
[5], [0 2 0 0 0 1 0], [120], [4], [6], [26], [0], [0], [true], [false], [0],
[5], [0 4 0 0 0 1 0], [4], [120], [6], [6], [0], [-10], [true], [false], [10],
[5], [1 4 0 0 0 1 0], [48], [10], [11], [31], [5], [15], [false], [false], [-],
[5], [2 4 0 0 0 1 0], [48], [10], [1], [31], [-5], [15], [false], [false], [-],
[5], [1 0 1 0 0 1 0], [240], [2], [5], [25], [-1], [0], [false], [false], [-],
[5], [2 0 1 0 0 1 0], [240], [2], [5], [35], [-1], [5], [false], [false], [-],
[5], [3 0 1 0 0 1 0], [240], [2], [5], [25], [-1], [0], [false], [false], [-],
[5], [4 0 1 0 0 1 0], [240], [2], [10], [30], [4], [10], [false], [false], [-],
[5], [1 0 2 0 0 1 0], [240], [2], [7], [35], [1], [5], [false], [false], [-],
[5], [2 0 2 0 0 1 0], [240], [2], [2], [30], [-4], [10], [false], [false], [-],
[5], [3 0 2 0 0 1 0], [240], [2], [7], [25], [1], [0], [false], [false], [-]
)

== Graph
#figure(image("../../figures/h7_subgroup.svg", width: 100%), caption: [H5 has no graph of its own (an exact census table); shown: the H7 graph of the same round.])
