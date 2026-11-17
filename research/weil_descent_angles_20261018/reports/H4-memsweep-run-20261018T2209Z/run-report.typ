#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: H4-memsweep-run-20261018T2209Z

Descriptive record of one recorded run of examples/n2\_routes.rs, mode memsweep (Addendum H4) under
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
commit 6083c58aa45e04e4d3194689b3b3e6ef5c7be115
rustc 1.97.0 (2d8144b78 2026-07-07)
4
Sun Oct 18 22:09:13 UTC 2026
host: this session's cloud container (L0)
cmd: n2_routes <dir> memsweep (N2_THREADS=4, sizes 32,36)
```
== memsweep\_summary.txt
```
H4 bits 32 theta 0.125: trials/p 0.7001 stored/p 0.00558 time*memory/p^2 0.00391 (40 runs)
H4 bits 32 theta 0.25: trials/p 0.6240 stored/p 0.02010 time*memory/p^2 0.01254 (40 runs)
H4 bits 32 theta 0.5: trials/p 0.5625 stored/p 0.07192 time*memory/p^2 0.04045 (40 runs)
H4 bits 36 theta 0.125: trials/p 0.6166 stored/p 0.00478 time*memory/p^2 0.00295 (40 runs)
H4 bits 36 theta 0.25: trials/p 0.5883 stored/p 0.01824 time*memory/p^2 0.01073 (40 runs)
H4 bits 36 theta 0.5: trials/p 0.4976 stored/p 0.06243 time*memory/p^2 0.03106 (40 runs)
H1/H4 all answers verified: true
```
== memsweep.csv (first 12 of 240 rows)
#table(columns: 17, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*bits*], [*h*], [*instance*], [*p*], [*ell*], [*cofactor\_exponent*], [*target*], [*arm*], [*theta*], [*steps*], [*relations*], [*stored\_rows*], [*verified*], [*model\_vertices*], [*model\_first\_cycle\_edges*], [*M\_total*], [*seconds*],
[32], [4], [0], [73291], [1342902107], [2], [0], [Jtheta], [0.125], [70182], [546], [545], [1], [0], [NaN], [3363480], [0.029289],
[32], [4], [0], [73291], [1342902107], [2], [0], [Jtheta], [0.25], [70159], [2218], [2217], [1], [0], [NaN], [3362440], [0.027032],
[32], [4], [0], [73291], [1342902107], [2], [0], [Jtheta], [0.5], [61720], [7949], [7948], [1], [0], [NaN], [2975117], [0.029596],
[32], [4], [0], [73291], [1342902107], [2], [1], [Jtheta], [0.125], [29140], [218], [217], [1], [0], [NaN], [1477312], [0.011393],
[32], [4], [0], [73291], [1342902107], [2], [1], [Jtheta], [0.25], [29079], [931], [930], [1], [0], [NaN], [1474562], [0.012031],
[32], [4], [0], [73291], [1342902107], [2], [1], [Jtheta], [0.5], [20745], [2631], [2630], [1], [0], [NaN], [1090426], [0.009932],
[32], [4], [0], [73291], [1342902107], [2], [2], [Jtheta], [0.125], [77930], [623], [622], [1], [0], [NaN], [3718434], [0.029318],
[32], [4], [0], [73291], [1342902107], [2], [2], [Jtheta], [0.25], [77930], [2508], [2507], [1], [0], [NaN], [3718435], [0.029932],
[32], [4], [0], [73291], [1342902107], [2], [2], [Jtheta], [0.5], [22413], [2830], [2829], [1], [0], [NaN], [1167650], [0.009705],
[32], [4], [0], [73291], [1342902107], [2], [3], [Jtheta], [0.125], [41382], [359], [358], [1], [0], [NaN], [2037996], [0.017888],
[32], [4], [0], [73291], [1342902107], [2], [3], [Jtheta], [0.25], [38086], [1266], [1265], [1], [0], [NaN], [1885475], [0.015026],
[32], [4], [0], [73291], [1342902107], [2], [3], [Jtheta], [0.5], [59608], [7550], [7549], [1], [0], [NaN], [2871841], [0.023823]
)

== Graph
#figure(image("../../figures/h4_memory_time.svg", width: 100%), caption: [H4: trials against stored relations as the distinguished fraction theta falls.])
