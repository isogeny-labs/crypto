#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: item1D-mult-run-20261018T2106Z

Descriptive record of one recorded run of examples/decomposition\_coverage.rs, mode mult (Amendment D) under
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
commit 44a81248a6aaf64ac4d5373e0c97867675887e2f
cmd: dc research/weil_descent_angles_20261018/results/item1D-mult-run-20261018T2106Z mult
rustc 1.97.0 (2d8144b78 2026-07-07)
Sun Oct 18 21:06:02 UTC 2026
post-run (d2edb2ab8): the tool was rustfmt-formatted (format only; unit tests pass); outputs unaffected.
```
== checks.txt
```
M1 identity x(P+Q)+x(P-Q) = x1x2/(x1+x2)^2 violations: 0
M2 solution cosets not a power of 2: 0
M3 cross-check distinct(subfield) != distinct from cosets: 0 curves
M3 dispersion char 0 null: mean 0.9923 sd 0.0213 over 15 curves
M3 dispersion char 0 subfield: mean 0.9725 sd 0.0320 over 15 curves
M3 dispersion char 2 null: mean 0.9576 sd 0.0955 over 21 curves
M3 dispersion char 2 subfield: mean 1.0007 sd 0.0301 over 21 curves
```
== binary\_cosets.csv (first 12 of 21 rows)
#table(columns: 24, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*m*], [*curve*], [*order*], [*m1\_pairs*], [*m1\_violations*], [*x3\_total*], [*solvable*], [*nonpow2\_cosets*], [*kdim0*], [*kdim1*], [*kdim2*], [*kdim3*], [*kdim4plus*], [*valid0*], [*valid1*], [*valid2*], [*valid3*], [*valid4*], [*valid5*], [*valid6*], [*valid7*], [*valid8plus*], [*distinct\_subfield*], [*distinct\_from\_cosets*],
[4], [0], [240], [28], [0], [256], [120], [0], [0], [240], [0], [0], [16], [79], [26], [15], [0], [0], [0], [0], [0], [0], [83], [83],
[4], [1], [234], [36], [0], [256], [120], [0], [0], [240], [0], [0], [16], [70], [28], [22], [0], [0], [0], [0], [0], [0], [101], [101],
[4], [2], [240], [28], [0], [256], [120], [0], [0], [240], [0], [0], [16], [79], [26], [15], [0], [0], [0], [0], [0], [0], [83], [83],
[5], [0], [990], [136], [0], [1024], [496], [0], [0], [992], [0], [0], [32], [291], [138], [67], [0], [0], [0], [0], [0], [0], [411], [411],
[5], [1], [990], [136], [0], [1024], [496], [0], [0], [992], [0], [0], [32], [300], [120], [76], [0], [0], [0], [0], [0], [0], [393], [393],
[5], [2], [1034], [136], [0], [1024], [496], [0], [0], [992], [0], [0], [32], [286], [148], [62], [0], [0], [0], [0], [0], [0], [421], [421],
[6], [0], [3982], [528], [0], [4096], [2016], [0], [0], [4032], [0], [0], [64], [1235], [506], [275], [0], [0], [0], [0], [0], [0], [1563], [1563],
[6], [1], [4034], [528], [0], [4096], [2016], [0], [0], [4032], [0], [0], [64], [1245], [486], [285], [0], [0], [0], [0], [0], [0], [1543], [1543],
[6], [2], [4096], [496], [0], [4096], [2016], [0], [0], [4032], [0], [0], [64], [1269], [502], [245], [0], [0], [0], [0], [0], [0], [1495], [1495],
[7], [0], [16492], [2016], [0], [16384], [8128], [0], [0], [16256], [0], [0], [128], [5075], [2074], [979], [0], [0], [0], [0], [0], [0], [6107], [6107],
[7], [1], [16230], [2080], [0], [16384], [8128], [0], [0], [16256], [0], [0], [128], [5016], [2064], [1048], [0], [0], [0], [0], [0], [0], [6225], [6225],
[7], [2], [16408], [2016], [0], [16384], [8128], [0], [0], [16256], [0], [0], [128], [5103], [2018], [1007], [0], [0], [0], [0], [0], [0], [6051], [6051]
)
== mult.csv (first 12 of 72 rows)
#table(columns: 19, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*char*], [*q*], [*curve*], [*order*], [*arm*], [*F*], [*lambda*], [*poisson*], [*pi*], [*dispersion*], [*h0*], [*h1*], [*h2*], [*h3*], [*h4*], [*h5*], [*h6*], [*h7*], [*h8plus*],
[2], [16], [0], [240], [subfield], [15], [0.500000], [0.393469], [0.345833], [1.06710], [157], [52], [30], [0], [0], [0], [0], [0], [0],
[2], [16], [0], [240], [null], [10], [0.229167], [0.204804], [0.212500], [0.79079], [189], [50], [0], [0], [0], [0], [0], [0], [0],
[2], [16], [1], [234], [subfield], [17], [0.653846], [0.479958], [0.431624], [0.99309], [133], [56], [44], [0], [0], [0], [0], [0], [0],
[2], [16], [1], [234], [null], [20], [0.897436], [0.592387], [0.636752], [0.82163], [85], [110], [26], [10], [2], [0], [0], [0], [0],
[2], [16], [2], [240], [subfield], [15], [0.500000], [0.393469], [0.345833], [1.06710], [157], [52], [30], [0], [0], [0], [0], [0], [0],
[2], [16], [2], [240], [null], [13], [0.379167], [0.315568], [0.333333], [0.76758], [160], [74], [5], [0], [0], [0], [0], [0], [0],
[2], [32], [0], [990], [subfield], [33], [0.566667], [0.432586], [0.415152], [0.94260], [579], [276], [134], [0], [0], [0], [0], [0], [0],
[2], [32], [0], [990], [null], [36], [0.672727], [0.489685], [0.510101], [0.83862], [485], [376], [112], [16], [0], [0], [0], [0], [0],
[2], [32], [1], [990], [subfield], [33], [0.566667], [0.432586], [0.396970], [1.00877], [597], [240], [152], [0], [0], [0], [0], [0], [0],
[2], [32], [1], [990], [null], [26], [0.354545], [0.298508], [0.308081], [0.87126], [685], [272], [30], [2], [0], [0], [0], [0], [0],
[2], [32], [2], [1034], [subfield], [33], [0.542553], [0.418738], [0.407157], [0.92926], [613], [296], [124], [0], [0], [0], [0], [0], [0],
[2], [32], [2], [1034], [null], [22], [0.244681], [0.217046], [0.192456], [1.16243], [835], [158], [36], [4], [0], [0], [0], [0], [0]
)

== Graph
#figure(image("../../figures/item1_multiplicity.svg", width: 100%), caption: [Amendment D: point multiplicity histograms, subfield vs null; exact.])
