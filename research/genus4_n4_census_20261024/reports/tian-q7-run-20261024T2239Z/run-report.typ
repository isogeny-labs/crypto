#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: tian-q7-run-20261024T2239Z

Descriptive record of one recorded run of examples/genus4\_n4\_census.rs, part TIAN under
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
run: tian-q7 20261024T2239Z
git HEAD: 7a579ce24983ef78f6af4ecf3dc9a8c6f10e8a91
tool: examples/genus4_n4_census.rs sha256 efce43f3fdaa848389a03f3d778a9b9604fe23d1d6e5e1bd75e777a49fb6cf01
protocol: research/genus4_n4_census_20261024/PROTOCOL.md sha256 65702f045ec07ff04edc651d7add4725ceecc7cf1bdc79b495a712a8cb942a44
command: /tmp/claude-0/g4/g4 research/genus4_n4_census_20261024/results/tian-q7-run-20261024T2239Z TIAN 7 /tmp/claude-0/g4/empty_hits.csv
rustc: rustc 1.97.0 (2d8144b78 2026-07-07)
host: Linux 6.18.44-fc-v153 x86_64, 4 cores, Intel(R) Xeon(R) Processor @ 2.10GHz
threads: 1
wall seconds: 1
```
== tool\_log.txt
```
t=-75 done (0s)
t=-71 done (0s)
t=-65 done (0s)
t=-57 done (0s)
t=-45 done (0s)
t=-39 done (1s)
t=-15 done (1s)
t=-9 done (1s)
t=3 done (1s)
t=9 done (1s)
t=13 done (1s)
t=19 done (1s)
t=25 done (1s)
t=31 done (1s)
t=45 done (1s)
t=51 done (1s)
t=55 done (1s)
t=61 done (1s)
t=69 done (1s)
t=93 done (1s)
```
== tian\_by\_t.csv (first 12 of 60 rows)
#table(columns: 14, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*q*], [*t*], [*nE*], [*hits\_forms*], [*ordinary*], [*disc*], [*ell*], [*qbar*], [*tr*], [*det*], [*types*], [*n\_ker\_by\_type*], [*n\_tian\_by\_type*], [*tian\_exists*],
[7], [-75], [2477], [0], [true], [-3979], [2], [1], [1], [1], [nonsplit], [1], [1], [yes],
[7], [-75], [2477], [0], [true], [-3979], [3], [1], [0], [1], [nonsplit], [2], [2], [yes],
[7], [-75], [2477], [0], [true], [-3979], [5], [2], [0], [1], [split], [2], [0], [no],
[7], [-71], [2473], [0], [true], [-4563], [2], [1], [1], [1], [nonsplit], [1], [1], [yes],
[7], [-71], [2473], [0], [true], [-4563], [3], [1], [1], [1], [scalar|jordan], [12|3], [8|2], [yes],
[7], [-71], [2473], [0], [true], [-4563], [5], [2], [4], [1], [nonsplit], [4], [4], [yes],
[7], [-65], [2467], [0], [true], [-5379], [2], [1], [1], [1], [nonsplit], [1], [1], [yes],
[7], [-65], [2467], [0], [true], [-5379], [3], [1], [1], [1], [jordan], [3], [2], [yes],
[7], [-65], [2467], [0], [true], [-5379], [5], [2], [0], [1], [split], [2], [0], [no],
[7], [-57], [2459], [0], [true], [-6355], [2], [1], [1], [1], [nonsplit], [1], [1], [yes],
[7], [-57], [2459], [0], [true], [-6355], [3], [1], [0], [1], [nonsplit], [2], [2], [yes],
[7], [-57], [2459], [0], [true], [-6355], [5], [2], [3], [1], [jordan], [1], [0], [no]
)

== Graph
#figure(image("../../figures/fig_tian.svg", width: 100%), caption: [Tian kernel existence per prime-order trace and l (n = 4).])
