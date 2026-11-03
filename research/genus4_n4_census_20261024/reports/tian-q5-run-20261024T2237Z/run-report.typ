#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: tian-q5-run-20261024T2237Z

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
run: tian-q5 20261024T2237Z
git HEAD: 7a579ce24983ef78f6af4ecf3dc9a8c6f10e8a91
tool: examples/genus4_n4_census.rs sha256 b1418307d05092210b128c9042719254b7db2e1d2817cec79d4c98bc1f935410
protocol: research/genus4_n4_census_20261024/PROTOCOL.md sha256 65702f045ec07ff04edc651d7add4725ceecc7cf1bdc79b495a712a8cb942a44
command: /tmp/claude-0/g4/g4 research/genus4_n4_census_20261024/results/tian-q5-run-20261024T2237Z TIAN 5 research/genus4_n4_census_20261024/results/cen-q5-cone-run-20261024T2208Z/hits.csv,research/genus4_n4_census_20261024/results/cen-q5-nonsplit-run-20261024T2210Z/hits.csv,research/genus4_n4_census_20261024/results/cen-q5-split-run-20261024T2206Z/hits.csv
rustc: rustc 1.97.0 (2d8144b78 2026-07-07)
host: Linux 6.18.44-fc-v153 x86_64, 4 cores, Intel(R) Xeon(R) Processor @ 2.10GHz
threads: 1
wall seconds: 9
```
== tool\_log.txt
```
t=-47 done (2s)
t=-33 done (2s)
t=-27 done (3s)
t=-21 done (5s)
t=-17 done (6s)
t=7 done (6s)
t=9 done (6s)
t=13 done (8s)
t=19 done (9s)
t=25 done (9s)
t=27 done (9s)
t=33 done (9s)
t=39 done (9s)
t=49 done (9s)
```
== tian\_by\_t.csv (first 12 of 42 rows)
#table(columns: 14, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*q*], [*t*], [*nE*], [*hits\_forms*], [*ordinary*], [*disc*], [*ell*], [*qbar*], [*tr*], [*det*], [*types*], [*n\_ker\_by\_type*], [*n\_tian\_by\_type*], [*tian\_exists*],
[5], [-47], [673], [28], [true], [-291], [2], [1], [1], [1], [nonsplit], [1], [1], [yes],
[5], [-47], [673], [28], [true], [-291], [3], [2], [1], [1], [jordan], [1], [0], [no],
[5], [-47], [673], [28], [true], [-291], [7], [5], [2], [2], [nonsplit], [2], [2], [yes],
[5], [-33], [659], [1037], [true], [-1411], [2], [1], [1], [1], [nonsplit], [1], [1], [yes],
[5], [-33], [659], [1037], [true], [-1411], [3], [2], [0], [1], [nonsplit], [2], [2], [yes],
[5], [-33], [659], [1037], [true], [-1411], [7], [5], [2], [2], [nonsplit], [2], [2], [yes],
[5], [-27], [653], [2036], [true], [-1771], [2], [1], [1], [1], [nonsplit], [1], [1], [yes],
[5], [-27], [653], [2036], [true], [-1771], [3], [2], [0], [1], [nonsplit], [2], [2], [yes],
[5], [-27], [653], [2036], [true], [-1771], [7], [5], [1], [2], [jordan], [3], [2], [yes],
[5], [-21], [647], [781], [true], [-2059], [2], [1], [1], [1], [nonsplit], [1], [1], [yes],
[5], [-21], [647], [781], [true], [-2059], [3], [2], [0], [1], [nonsplit], [2], [2], [yes],
[5], [-21], [647], [781], [true], [-2059], [7], [5], [0], [2], [nonsplit], [0], [0], [no]
)

== Graph
#figure(image("../../figures/fig_tian.svg", width: 100%), caption: [Tian kernel existence per prime-order trace and l (n = 4).])
