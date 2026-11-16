#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: tiantheory-run-20261024T2225Z

Descriptive record of one recorded run of examples/genus4\_n4\_census.rs, part TIANTHEORY under
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
run: tiantheory 20261024T2225Z
git HEAD: 7a579ce24983ef78f6af4ecf3dc9a8c6f10e8a91
tool: examples/genus4_n4_census.rs sha256 cc0bcbe7b428feff4e2acbea3c6887d6abb5031b01f3a49589bd3597b3773dc7
protocol: research/genus4_n4_census_20261024/PROTOCOL.md sha256 457dc80ceded9acbbb0fb39ca1c61e8c3e2bfa773f863100584969c76131187d
command: /tmp/claude-0/g4/g4 research/genus4_n4_census_20261024/results/tiantheory-run-20261024T2225Z TIANTHEORY 3,5,7
rustc: rustc 1.97.0 (2d8144b78 2026-07-07)
host: Linux 6.18.44-fc-v153 x86_64, 4 cores, Intel(R) Xeon(R) Processor @ 2.10GHz
threads: 1
wall seconds: 84
```
== tool\_log.txt
```
theory check l=3 done
theory check l=5 done
theory check l=7 done
Theorem 6 cells compared: 44; disagreements: 0
```
== non\_regular\_cells.txt
```
l=3 qbar=1 tr=1 scalar: n_ker=12 n_tian=8
l=3 qbar=1 tr=1 jordan: n_ker=3 n_tian=2
l=3 qbar=1 tr=2 scalar: n_ker=64 n_tian=60
l=3 qbar=1 tr=2 jordan: n_ker=1 n_tian=0
l=3 qbar=2 tr=1 scalar: n_ker=16 n_tian=12
l=3 qbar=2 tr=1 jordan: n_ker=1 n_tian=0
l=3 qbar=2 tr=2 scalar: n_ker=60 n_tian=56
l=3 qbar=2 tr=2 jordan: n_ker=3 n_tian=2
l=5 qbar=1 tr=2 scalar: n_ker=288 n_tian=282
l=5 qbar=1 tr=2 jordan: n_ker=3 n_tian=2
l=5 qbar=1 tr=3 scalar: n_ker=28 n_tian=22
l=5 qbar=1 tr=3 jordan: n_ker=3 n_tian=2
l=5 qbar=2 tr=2 scalar: n_ker=64 n_tian=58
l=5 qbar=2 tr=2 jordan: n_ker=9 n_tian=8
l=5 qbar=2 tr=3 scalar: n_ker=156 n_tian=150
l=5 qbar=2 tr=3 jordan: n_ker=1 n_tian=0
l=5 qbar=3 tr=2 scalar: n_ker=64 n_tian=58
l=5 qbar=3 tr=2 jordan: n_ker=9 n_tian=8
l=5 qbar=3 tr=3 scalar: n_ker=156 n_tian=150
l=5 qbar=3 tr=3 jordan: n_ker=1 n_tian=0
l=5 qbar=4 tr=2 scalar: n_ker=288 n_tian=282
l=5 qbar=4 tr=2 jordan: n_ker=3 n_tian=2
l=5 qbar=4 tr=3 scalar: n_ker=28 n_tian=22
l=5 qbar=4 tr=3 jordan: n_ker=3 n_tian=2
l=7 qbar=1 tr=2 scalar: n_ker=512 n_tian=504
l=7 qbar=1 tr=2 jordan: n_ker=1 n_tian=0
l=7 qbar=1 tr=5 scalar: n_ker=64 n_tian=56
l=7 qbar=1 tr=5 jordan: n_ker=1 n_tian=0
l=7 qbar=2 tr=1 scalar: n_ker=512 n_tian=504
l=7 qbar=2 tr=1 jordan: n_ker=1 n_tian=0
l=7 qbar=2 tr=6 scalar: n_ker=64 n_tian=56
l=7 qbar=2 tr=6 jordan: n_ker=1 n_tian=0
l=7 qbar=3 tr=3 scalar: n_ker=52 n_tian=44
l=7 qbar=3 tr=3 jordan: n_ker=3 n_tian=2
l=7 qbar=3 tr=4 scalar: n_ker=500 n_tian=492
l=7 qbar=3 tr=4 jordan: n_ker=3 n_tian=2
l=7 qbar=4 tr=3 scalar: n_ker=64 n_tian=56
l=7 qbar=4 tr=3 jordan: n_ker=1 n_tian=0
l=7 qbar=4 tr=4 scalar: n_ker=512 n_tian=504
l=7 qbar=4 tr=4 jordan: n_ker=1 n_tian=0
l=7 qbar=5 tr=1 scalar: n_ker=500 n_tian=492
l=7 qbar=5 tr=1 jordan: n_ker=3 n_tian=2
l=7 qbar=5 tr=6 scalar: n_ker=52 n_tian=44
l=7 qbar=5 tr=6 jordan: n_ker=3 n_tian=2
l=7 qbar=6 tr=2 scalar: n_ker=500 n_tian=492
l=7 qbar=6 tr=2 jordan: n_ker=3 n_tian=2
l=7 qbar=6 tr=5 scalar: n_ker=52 n_tian=44
l=7 qbar=6 tr=5 jordan: n_ker=3 n_tian=2
```
== summary.txt
```
Theorem 6 cells compared: 44; disagreements: 0
```
== tian\_theory.csv (first 12 of 92 rows)
#table(columns: 9, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*ell*], [*qbar*], [*tr*], [*det*], [*class*], [*theory*], [*linear\_algebra*], [*n\_ker*], [*agree*],
[3], [1], [0], [1], [nonsplit], [2], [2], [2], [true],
[3], [1], [1], [1], [scalar], [-], [8], [12], [true],
[3], [1], [1], [1], [jordan], [-], [2], [3], [true],
[3], [1], [2], [1], [scalar], [-], [60], [64], [true],
[3], [1], [2], [1], [jordan], [-], [0], [1], [true],
[3], [2], [0], [1], [nonsplit], [2], [2], [2], [true],
[3], [2], [1], [1], [scalar], [-], [12], [16], [true],
[3], [2], [1], [1], [jordan], [-], [0], [1], [true],
[3], [2], [2], [1], [scalar], [-], [56], [60], [true],
[3], [2], [2], [1], [jordan], [-], [2], [3], [true],
[5], [1], [0], [1], [split], [0], [0], [2], [true],
[5], [1], [1], [1], [nonsplit], [4], [4], [4], [true]
)

== Graph
#figure(image("../../figures/fig_tian.svg", width: 100%), caption: [Tian kernel existence per prime-order trace and l (n = 4).])
