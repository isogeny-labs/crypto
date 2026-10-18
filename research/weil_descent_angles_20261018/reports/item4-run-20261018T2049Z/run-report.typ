#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: item4-run-20261018T2049Z

Descriptive record of one recorded run of examples/genus4\_res\_census.rs (Parts T, H4, K) under
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
run: item 4, 20261018T2049Z (Parts T, H4, K)
protocol commit: bdcf8f8ad (item 4 frozen); tool commit: 252b52998
rustc 1.97.0 (2d8144b78 2026-07-07)
host: Linux 6.18.44-fc-v139 x86_64, 4 cores, Intel(R) Xeon(R) Processor @ 2.10GHz, 15 GB
command: rustc --edition 2021 -O examples/genus4_res_census.rs -o g4 && G4_THREADS=4 ./g4 research/weil_descent_angles_20261018/results/item4-run-20261018T2049Z T H4 K
tool sha256: 8c6597ea8d5eb73604f8f2d3d724df4275deefce75db8944c6dc2c2b6edcc27b
threads: 4
smoke run before this run (not evidence): G4_LIMIT=200000 (4 s), Parts T and H4 as here; the K slice showed the expected orbit-outside-slice K5 failures only.
```
== tool\_log.txt
```
partT: n with an odd-t solution and n >= 3: [] (P5 predicts none)
partT: n = 2 odd-t cycle types: [6] [3, 3]
partT: n = 4 even-t solutions all in powers of 2: true
field F_3^12 ready 0.2s
H4: squarefree degree-10 binary forms = 104976; mass = 104976/48 = 2187 (check K1 expects 2187: ok)
H4: Res-shaped models = 1100; t histogram {-18: 18, -10: 96, -8: 144, -6: 96, -2: 192, 0: 204, 2: 180, 4: 48, 8: 48, 10: 48, 16: 24, 18: 2}
H4: check K2 (t even and cycle types in powers of 2) violations = 0
classify split: 36 classes, K5 violations 0
part K split: forms 43046720 smooth 21830400 mass 9475.0000 res_forms 81216 classes 36 (416.4s)
classify nonsplit: 39 classes, K5 violations 0
part K nonsplit: forms 43046720 smooth 27299520 mass 9479.0000 res_forms 110880 classes 39 (714.3s)
classify cone: 83 classes, K5 violations 0
part K cone: forms 14348906 smooth 8503056 mass 6561.0000 res_forms 101520 classes 83 (267.0s)
K: non-hyperelliptic mass total = 25515.000000 (check K3 literature value q^9+q^8-q^6 = 25515: match)
K: Res-shaped classes = 158; t histogram (class counts) {-18: 3, -17: 2, -16: 3, -15: 1, -14: 8, -13: 6, -12: 2, -11: 7, -10: 3, -9: 1, -8: 14, -7: 3, -6: 11, -5: 9, -4: 5, -3: 15, -2: 11, -1: 2, 0: 4, 1: 4, 2: 17, 3: 2, 4: 9, 5: 2, 7: 6, 9: 3, 10: 4, 11: 1}
K: check K4 violations = 0; check K5 violations = 0
K: P6 prime-order classes (82 - t prime, elliptic trace exists) = 9 : ["split:t=11:#E=71:aut=1", "nonsplit:t=-1:#E=83:aut=1", "cone:t=-7:#E=89:aut=1", "cone:t=9:#E=73:aut=3", "cone:t=9:#E=73:aut=1", "cone:t=9:#E=73:aut=1", "cone:t=-1:#E=83:aut=1", "cone:t=-7:#E=89:aut=1", "cone:t=-7:#E=89:aut=1"]
total 1398.1s
```
== checks.txt
```
partT: n with an odd-t solution and n >= 3: [] (P5 predicts none)
partT: n = 2 odd-t cycle types: [6] [3, 3]
partT: n = 4 even-t solutions all in powers of 2: true
H4: squarefree degree-10 binary forms = 104976; mass = 104976/48 = 2187 (check K1 expects 2187: ok)
H4: Res-shaped models = 1100; t histogram {-18: 18, -10: 96, -8: 144, -6: 96, -2: 192, 0: 204, 2: 180, 4: 48, 8: 48, 10: 48, 16: 24, 18: 2}
H4: check K2 (t even and cycle types in powers of 2) violations = 0
K: non-hyperelliptic mass total = 25515.000000 (check K3 literature value q^9+q^8-q^6 = 25515: match)
K: Res-shaped classes = 158; t histogram (class counts) {-18: 3, -17: 2, -16: 3, -15: 1, -14: 8, -13: 6, -12: 2, -11: 7, -10: 3, -9: 1, -8: 14, -7: 3, -6: 11, -5: 9, -4: 5, -3: 15, -2: 11, -1: 2, 0: 4, 1: 4, 2: 17, 3: 2, 4: 9, 5: 2, 7: 6, 9: 3, 10: 4, 11: 1}
K: check K4 violations = 0; check K5 violations = 0
K: P6 prime-order classes (82 - t prime, elliptic trace exists) = 9 : ["split:t=11:#E=71:aut=1", "nonsplit:t=-1:#E=83:aut=1", "cone:t=-7:#E=89:aut=1", "cone:t=9:#E=73:aut=3", "cone:t=9:#E=73:aut=1", "cone:t=9:#E=73:aut=1", "cone:t=-1:#E=83:aut=1", "cone:t=-7:#E=89:aut=1", "cone:t=-7:#E=89:aut=1"]
```
== h4\_res\_shaped.csv (first 12 of 1100 rows)
#table(columns: 7, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*code*], [*a0..a10*], [*N4*], [*t*], [*t\_even*], [*cycle\_type*], [*cycles\_powers\_of\_2*],
[19686], [0 1 0 0 0 0 0 0 0 1 0], [154], [-18], [true], [1+1+4+4], [true],
[19687], [1 1 0 0 0 0 0 0 0 1 0], [154], [-18], [true], [1+1+4+4], [true],
[19688], [2 1 0 0 0 0 0 0 0 1 0], [154], [-18], [true], [1+1+4+4], [true],
[19689], [0 2 0 0 0 0 0 0 0 1 0], [10], [18], [true], [1+1+1+1+2+2+2], [true],
[19716], [0 2 0 1 0 0 0 0 0 1 0], [82], [0], [true], [1+1+8], [true],
[19717], [1 2 0 1 0 0 0 0 0 1 0], [82], [0], [true], [1+1+8], [true],
[19718], [2 2 0 1 0 0 0 0 0 1 0], [82], [0], [true], [1+1+8], [true],
[19743], [0 2 0 2 0 0 0 0 0 1 0], [82], [0], [true], [1+1+8], [true],
[19744], [1 2 0 2 0 0 0 0 0 1 0], [82], [0], [true], [1+1+8], [true],
[19745], [2 2 0 2 0 0 0 0 0 1 0], [82], [0], [true], [1+1+8], [true],
[19932], [0 2 0 0 0 1 0 0 0 1 0], [18], [16], [true], [1+1+8], [true],
[19960], [1 2 0 1 0 1 0 0 0 1 0], [90], [-2], [true], [1+1+8], [true]
)
== k\_classes.csv (first 12 of 158 rows)
#table(columns: 10, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*rep\_code*], [*orbit\_forms*], [*aut*], [*N4*], [*t*], [*numE*], [*numE\_prime*], [*elliptic\_trace\_exists*], [*form*],
[split], [708734], [2304], [1], [94], [-3], [85], [false], [false], [2\*x^0y^0 + 1\*x^0y^2 + 2\*x^0y^3 + 1\*x^1y^0 + 1\*x^2y^3 + 1\*x^3y^0],
[split], [708761], [2304], [1], [106], [-6], [88], [false], [false], [2\*x^0y^0 + 1\*x^0y^2 + 2\*x^1y^0 + 1\*x^2y^3 + 1\*x^3y^0],
[split], [708989], [2304], [1], [106], [-6], [88], [false], [false], [2\*x^0y^0 + 1\*x^0y^1 + 2\*x^0y^2 + 2\*x^0y^3 + 1\*x^1y^0 + 1\*x^1y^1 + 1\*x^2y^3 + 1\*x^3y^0],
[split], [728311], [2304], [1], [114], [-8], [90], [false], [true], [1\*x^0y^0 + 1\*x^0y^1 + 1\*x^0y^2 + 1\*x^0y^3 + 1\*x^2y^1 + 1\*x^2y^3 + 1\*x^3y^0],
[split], [729470], [2304], [1], [90], [-2], [84], [false], [true], [2\*x^0y^0 + 1\*x^0y^2 + 2\*x^0y^3 + 2\*x^1y^0 + 1\*x^1y^1 + 1\*x^1y^2 + 1\*x^2y^1 + 1\*x^2y^3 + 1\*x^3y^0],
[split], [729949], [2304], [1], [102], [-5], [87], [false], [true], [1\*x^0y^0 + 1\*x^0y^1 + 2\*x^0y^3 + 2\*x^1y^0 + 2\*x^1y^2 + 1\*x^2y^1 + 1\*x^2y^3 + 1\*x^3y^0],
[split], [755479], [2304], [1], [54], [7], [75], [false], [true], [1\*x^0y^0 + 2\*x^0y^2 + 2\*x^0y^3 + 2\*x^1y^0 + 1\*x^1y^2 + 1\*x^2y^0 + 2\*x^2y^1 + 1\*x^2y^3 + 1\*x^3y^0],
[split], [768113], [2304], [1], [78], [1], [81], [false], [true], [2\*x^0y^0 + 2\*x^0y^1 + 1\*x^0y^2 + 2\*x^0y^3 + 2\*x^1y^0 + 1\*x^1y^1 + 1\*x^2y^2 + 1\*x^2y^3 + 1\*x^3y^0],
[split], [768506], [2304], [1], [94], [-3], [85], [false], [false], [2\*x^0y^0 + 1\*x^0y^1 + 2\*x^0y^3 + 1\*x^1y^0 + 1\*x^1y^2 + 1\*x^2y^2 + 1\*x^2y^3 + 1\*x^3y^0],
[split], [769274], [2304], [1], [66], [4], [78], [false], [true], [2\*x^0y^0 + 2\*x^0y^1 + 1\*x^0y^2 + 2\*x^1y^0 + 2\*x^1y^2 + 1\*x^2y^2 + 1\*x^2y^3 + 1\*x^3y^0],
[split], [769336], [2304], [1], [94], [-3], [85], [false], [false], [1\*x^0y^0 + 2\*x^0y^1 + 2\*x^0y^2 + 2\*x^0y^3 + 2\*x^1y^0 + 2\*x^1y^2 + 1\*x^2y^2 + 1\*x^2y^3 + 1\*x^3y^0],
[split], [774323], [2304], [1], [38], [11], [71], [true], [true], [2\*x^0y^0 + 2\*x^0y^1 + 1\*x^0y^2 + 1\*x^0y^3 + 1\*x^1y^0 + 1\*x^2y^0 + 1\*x^2y^2 + 1\*x^2y^3 + 1\*x^3y^0]
)
== k\_summary.csv (first 3 of 3 rows)
#table(columns: 10, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*forms*], [*smooth*], [*mass\_den*], [*mass*], [*smooth\_with\_N1\_4*], [*res\_shaped\_forms*], [*res\_shaped\_classes*], [*k5\_violations*], [*seconds*],
[split], [43046720], [21830400], [2304], [9475.000000], [4100256], [81216], [36], [0], [416.4],
[nonsplit], [43046720], [27299520], [2880], [9479.000000], [5611680], [110880], [39], [0], [714.3],
[cone], [14348906], [8503056], [1296], [6561.000000], [1925208], [101520], [83], [0], [267.0]
)
== part\_t.csv (first 12 of 30 rows)
#table(columns: 4, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*n*], [*t\_parity*], [*solutions*], [*cycle\_types*],
[2], [odd], [2], [6 3+3],
[2], [even], [6], [4+2 4+1+1 2+2+2 2+2+1+1 2+1+1+1+1 1+1+1+1+1+1],
[3], [odd], [0], [],
[3], [even], [4], [6+2 6+1+1 3+3+2 3+3+1+1],
[4], [odd], [0], [],
[4], [even], [14], [8+2 8+1+1 4+4+2 4+4+1+1 4+2+2+2 4+2+2+1+1 4+2+1+1+1+1 4+1+1+1+1+1+1 2+2+2+2+2 2+2+2+2+1+1 2+2+2+1+1+1+1 2+2+1+1+1+1+1+1 2+1+1+1+1+1+1+1+1 1+1+1+1+1+1+1+1+1+1],
[5], [odd], [0], [],
[5], [even], [4], [10+2 10+1+1 5+5+2 5+5+1+1],
[6], [odd], [0], [],
[6], [even], [8], [12+2 12+1+1 6+6+2 6+6+1+1 6+3+3+2 6+3+3+1+1 3+3+3+3+2 3+3+3+3+1+1],
[7], [odd], [0], [],
[7], [even], [4], [14+2 14+1+1 7+7+2 7+7+1+1]
)

== Graph
#figure(image("../../figures/item4_traces.svg", width: 100%), caption: [Res-shaped genus-4 curves over F\_3 by trace: hyperelliptic models (left), non-hyperelliptic classes (right); exact census.])
