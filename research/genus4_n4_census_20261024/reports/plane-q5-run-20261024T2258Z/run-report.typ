#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: plane-q5-run-20261024T2258Z

Descriptive record of one recorded run of examples/genus4\_n4\_census.rs, part PLANE under
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
run: plane-q5 20261024T2258Z
git HEAD: b08305739b2f95bb83966c2830da5e8d14eb2d92
tool: examples/genus4_n4_census.rs sha256 28e74764bccc9faa9b72ac80a9df0054c5e70996f71f7372615f5845afce2707
protocol: research/genus4_n4_census_20261024/PROTOCOL.md sha256 b0eba35fccb202d3a1dfa63c947f4a86997e40a9ee8dfa300b55d500bec43f3b
command: /tmp/claude-0/g4/g4 research/genus4_n4_census_20261024/results/plane-q5-run-20261024T2258Z PLANE 5 research/genus4_n4_census_20261024/results/cen-q5-cone-run-20261024T2208Z/hits.csv,research/genus4_n4_census_20261024/results/cen-q5-nonsplit-run-20261024T2210Z/hits.csv,research/genus4_n4_census_20261024/results/cen-q5-split-run-20261024T2206Z/hits.csv 2 3
rustc: rustc 1.97.0 (2d8144b78 2026-07-07)
host: Linux 6.18.44-fc-v153 x86_64, 4 cores, Intel(R) Xeon(R) Processor @ 2.10GHz
threads: 1
wall seconds: 1
```
== tool\_log.txt
```
plane quintic models checked: curves=68 models=408 (k,model) checks=1224 count_mismatches=0 degenerate_nodes=0 singular_set_mismatches=0
```
== summary.txt
```
plane quintic models checked: curves=68 models=408 (k,model) checks=1224 count_mismatches=0 degenerate_nodes=0 singular_set_mismatches=0
```
== plane\_models.csv (first 12 of 68 rows)
#table(columns: 11, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*t*], [*code*], [*npoints\_rational*], [*models\_checked*], [*mismatches*], [*degenerate*], [*sing\_mismatch*], [*nodes\_k1*], [*nodes\_k2*], [*nodes\_k3*],
[cone], [-17], [9985578608], [6], [6], [0], [0], [0], [6], [6], [6],
[cone], [-21], [4176399056], [6], [6], [0], [0], [0], [6], [6], [6],
[cone], [-27], [10209046869], [6], [6], [0], [0], [0], [6], [6], [6],
[cone], [-33], [27730488661], [6], [6], [0], [0], [0], [6], [6], [6],
[cone], [-21], [2615248492], [6], [6], [0], [0], [0], [6], [6], [6],
[cone], [27], [17160923437], [6], [6], [0], [0], [0], [6], [6], [6],
[cone], [9], [3637424030], [6], [6], [0], [0], [0], [6], [6], [6],
[cone], [-33], [12136877657], [6], [6], [0], [0], [0], [6], [6], [6],
[cone], [-27], [21965452193], [6], [6], [0], [0], [0], [6], [6], [6],
[cone], [-17], [15189766298], [6], [6], [0], [0], [0], [6], [6], [6],
[cone], [27], [18032064696], [6], [6], [0], [0], [0], [6], [6], [6],
[cone], [25], [11154693818], [6], [6], [0], [0], [0], [6], [6], [6]
)

== Graph
#figure(image("../../figures/fig_mass_by_t.svg", width: 100%), caption: [Mass of hits per prime-order trace with the Haar-density fit.])
