#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: classes-q3-cone-run-20261024T2205Z

Descriptive record of one recorded run of examples/genus4\_n4\_census.rs, part CLASSES under
`research/genus4_n4_census_20261024/PROTOCOL.md` (with its dated amendments). It lists what
ran, on what host, and what the tool recorded, file by file. Interpretation
is in the study's `REPORT.md`. Costs are counted operations
(F#sub[p] multiplications and inversions, group operations, native
operations), never wall-clock; the only times are the tool's own seconds.
Isolation: none earned (a shared cloud container, or a RunPod CPU pod whose lscpu is in host.txt); this
affects only the seconds, since every recorded value is an exact count or
a counted operation total.

== env.txt
```
run: classes-q3-cone 20261024T2205Z
git HEAD: 7a579ce24983ef78f6af4ecf3dc9a8c6f10e8a91
tool: examples/genus4_n4_census.rs sha256 cc0bcbe7b428feff4e2acbea3c6887d6abb5031b01f3a49589bd3597b3773dc7
protocol: research/genus4_n4_census_20261024/PROTOCOL.md sha256 457dc80ceded9acbbb0fb39ca1c61e8c3e2bfa773f863100584969c76131187d
command: /tmp/claude-0/g4/g4 research/genus4_n4_census_20261024/results/classes-q3-cone-run-20261024T2205Z CLASSES 3 research/genus4_n4_census_20261024/results/ctrlK-q3-cone-run-20261024T2151Z/hits.csv
rustc: rustc 1.97.0 (2d8144b78 2026-07-07)
host: Linux 6.18.44-fc-v153 x86_64, 4 cores, Intel(R) Xeon(R) Processor @ 2.10GHz
threads: 1
wall seconds: 0
```
== tool\_log.txt
```
type,t,nE,classes,mass
cone,-7,89,3,3.0000
cone,-1,83,1,1.0000
cone,9,73,3,2.3333
total classes 7
```
== summary.txt
```
type,t,nE,classes,mass
cone,-7,89,3,3.0000
cone,-1,83,1,1.0000
cone,9,73,3,2.3333
total classes 7
```
== classes.csv (first 7 of 7 rows)
#table(columns: 6, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*t*], [*nE*], [*orbit\_size*], [*aut*], [*rep\_code*],
[cone], [-7], [89], [1296], [1], [24310],
[cone], [9], [73], [432], [3], [70848],
[cone], [9], [73], [1296], [1], [75195],
[cone], [9], [73], [1296], [1], [179485],
[cone], [-1], [83], [1296], [1], [180770],
[cone], [-7], [89], [1296], [1], [225291],
[cone], [-7], [89], [1296], [1], [240613]
)

== Graph
#figure(image("../../figures/fig_mass_by_t.svg", width: 100%), caption: [Mass of hits per prime-order trace with the Haar-density fit.])
