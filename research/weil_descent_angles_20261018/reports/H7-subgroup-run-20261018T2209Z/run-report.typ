#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: H7-subgroup-run-20261018T2209Z

Descriptive record of one recorded run of examples/decomposition\_coverage.rs, mode subgroup (Addendum H7) under
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
cmd: decomposition_coverage <dir> subgroup
```
== checks.txt
```
H7 binary n=2: ell-subgroup coverage ratio subfield/null mean 1.864 (s.e. 0.155, 21 curves)
H7 binary n=4: ell-subgroup coverage ratio subfield/null mean 2.069 (s.e. 0.714, 9 curves)
H7 odd_generic n=2: ell-subgroup coverage ratio subfield/null mean 1.168 (s.e. 0.103, 12 curves)
H7 odd_generic n=4: ell-subgroup coverage ratio subfield/null mean 1.332 (s.e. 0.533, 9 curves)
H7 odd_split_twist n=2: ell-subgroup coverage ratio subfield/null mean 1.180 (s.e. 0.550, 12 curves)
H7 odd_split_twist n=4: ell-subgroup coverage ratio subfield/null mean 0.402 (s.e. 0.150, 9 curves)
H7 odd_split_twist_matched n=2: ell-subgroup coverage ratio subfield/null mean 2.393 (s.e. 0.214, 12 curves)
H7 odd_split_twist_matched n=4: ell-subgroup coverage ratio subfield/null mean 0.742 (s.e. 0.130, 9 curves)
```
== subgroup.csv (first 12 of 415 rows)
#table(columns: 14, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*kind*], [*n*], [*q*], [*curve*], [*order*], [*ell*], [*cofactor*], [*layer*], [*layer\_size*], [*F\_sub*], [*F\_null*], [*cov\_sub*], [*cov\_null*], [*ratio*],
[binary], [2], [16], [0], [276], [23], [12], [E], [276], [15], [16], [0.322464], [0.409420], [0.78761],
[binary], [2], [16], [0], [276], [23], [12], [2E], [138], [15], [16], [0.644928], [0.514493], [1.25352],
[binary], [2], [16], [0], [276], [23], [12], [4E], [69], [15], [16], [0.681159], [0.449275], [1.51613],
[binary], [2], [16], [0], [276], [23], [12], [ell], [23], [15], [16], [0.652174], [0.565217], [1.15385],
[binary], [2], [16], [1], [242], [11], [22], [E], [242], [17], [18], [0.442149], [0.413223], [1.07000],
[binary], [2], [16], [1], [242], [11], [22], [2E], [121], [17], [18], [0.884298], [0.438017], [2.01887],
[binary], [2], [16], [1], [242], [11], [22], [ell], [11], [17], [18], [0.818182], [0.818182], [1.00000],
[binary], [2], [16], [2], [270], [5], [54], [E], [270], [17], [12], [0.403704], [0.270370], [1.49315],
[binary], [2], [16], [2], [270], [5], [54], [2E], [135], [17], [12], [0.807407], [0.303704], [2.65854],
[binary], [2], [16], [2], [270], [5], [54], [ell], [5], [17], [12], [1.000000], [0.600000], [1.66667],
[binary], [2], [32], [0], [1024], [2], [512], [E], [1024], [31], [28], [0.356445], [0.338867], [1.05187],
[binary], [2], [32], [0], [1024], [2], [512], [2E], [512], [31], [28], [0.712891], [0.431641], [1.65158]
)

== Graph
#figure(image("../../figures/h7_subgroup.svg", width: 100%), caption: [H7: coverage of the l-subgroup, subfield over null, mean and 1.96 s.e. per cell.])
