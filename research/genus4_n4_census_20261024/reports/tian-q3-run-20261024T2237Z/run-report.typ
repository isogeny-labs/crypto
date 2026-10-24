#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: tian-q3-run-20261024T2237Z

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
run: tian-q3 20261024T2237Z
git HEAD: 7a579ce24983ef78f6af4ecf3dc9a8c6f10e8a91
tool: examples/genus4_n4_census.rs sha256 b1418307d05092210b128c9042719254b7db2e1d2817cec79d4c98bc1f935410
protocol: research/genus4_n4_census_20261024/PROTOCOL.md sha256 65702f045ec07ff04edc651d7add4725ceecc7cf1bdc79b495a712a8cb942a44
command: /tmp/claude-0/g4/g4 research/genus4_n4_census_20261024/results/tian-q3-run-20261024T2237Z TIAN 3 research/genus4_n4_census_20261024/results/ctrlK-q3-cone-run-20261024T2151Z/hits.csv,research/genus4_n4_census_20261024/results/ctrlK-q3-nonsplit-run-20261024T2153Z/hits.csv,research/genus4_n4_census_20261024/results/ctrlK-q3-split-run-20261024T2152Z/hits.csv
rustc: rustc 1.97.0 (2d8144b78 2026-07-07)
host: Linux 6.18.44-fc-v153 x86_64, 4 cores, Intel(R) Xeon(R) Processor @ 2.10GHz
threads: 1
wall seconds: 7
```
== tool\_log.txt
```
t=-7 done (2s)
t=-1 done (4s)
t=9 done (5s)
t=11 done (7s)
```
== tian\_by\_t.csv (first 12 of 12 rows)
#table(columns: 14, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*q*], [*t*], [*nE*], [*hits\_forms*], [*ordinary*], [*disc*], [*ell*], [*qbar*], [*tr*], [*det*], [*types*], [*n\_ker\_by\_type*], [*n\_tian\_by\_type*], [*tian\_exists*],
[3], [-7], [89], [3888], [true], [-275], [2], [1], [1], [1], [nonsplit], [1], [1], [yes],
[3], [-7], [89], [3888], [true], [-275], [5], [3], [3], [1], [scalar|jordan], [156|1], [150|0], [depends],
[3], [-7], [89], [3888], [true], [-275], [7], [3], [0], [4], [nonsplit], [0], [0], [no],
[3], [-1], [83], [4176], [true], [-323], [2], [1], [1], [1], [nonsplit], [1], [1], [yes],
[3], [-1], [83], [4176], [true], [-323], [5], [3], [4], [1], [nonsplit], [4], [4], [yes],
[3], [-1], [83], [4176], [true], [-323], [7], [3], [6], [4], [nonsplit], [2], [2], [yes],
[3], [9], [73], [3024], [false], [-243], [2], [1], [1], [1], [nonsplit], [1], [1], [yes],
[3], [9], [73], [3024], [false], [-243], [5], [3], [4], [1], [nonsplit], [4], [4], [yes],
[3], [9], [73], [3024], [false], [-243], [7], [3], [2], [4], [split], [4], [2], [yes],
[3], [11], [71], [2304], [true], [-203], [2], [1], [1], [1], [nonsplit], [1], [1], [yes],
[3], [11], [71], [2304], [true], [-203], [5], [3], [1], [1], [nonsplit], [0], [0], [no],
[3], [11], [71], [2304], [true], [-203], [7], [3], [4], [4], [jordan], [3], [2], [yes]
)

== Graph
#figure(image("../../figures/fig_tian.svg", width: 100%), caption: [Tian kernel existence per prime-order trace and l (n = 4).])
