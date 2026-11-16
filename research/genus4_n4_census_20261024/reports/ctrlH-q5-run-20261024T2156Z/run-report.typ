#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: ctrlH-q5-run-20261024T2156Z

Descriptive record of one recorded run of examples/genus4\_n4\_census.rs, part HYPER under
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
run: ctrlH-q5 20261024T2156Z
git HEAD: 0fb870d0ee76ba6a1b059e9e34c869518fb412b0
tool: examples/genus4_n4_census.rs sha256 f4c7f7355c838b6f112e09cb412fd3353d087309962c64cd39ad49d79f4c9dad
protocol: research/genus4_n4_census_20261024/PROTOCOL.md sha256 a95b1e35748cefe23a044d8cfd9e059b723cf71fdbc4af56db71257da42c8b33
command: /tmp/claude-0/g4/g4 research/genus4_n4_census_20261024/results/ctrlH-q5-run-20261024T2156Z HYPER 5 50000000 202610245
rustc: rustc 1.97.0 (2d8144b78 2026-07-07)
host: Linux 6.18.44-fc-v153 x86_64, 4 cores, Intel(R) Xeon(R) Processor @ 2.10GHz
threads: 1
wall seconds: 95
```
== tool\_log.txt
```
HYPER q=5 mode=sampled forms_examined=50000000
squarefree_degree10_forms=38397517
res_shaped=119496
odd_t_hits=0
t_hist={-50: 184, -46: 477, -44: 528, -36: 989, -34: 7507, -32: 2461, -30: 960, -28: 1037, -26: 1444, -24: 4309, -22: 8287, -20: 1435, -18: 3420, -16: 1916, -14: 17066, -12: 2030, -10: 5422, -6: 1971, -4: 10316, -2: 537, 0: 1157, 2: 8590, 4: 1497, 6: 4420, 10: 3041, 12: 1967, 14: 7564, 16: 461, 18: 3713, 20: 1921, 22: 2008, 24: 529, 26: 5424, 32: 2148, 34: 497, 36: 489, 38: 971, 46: 604, 50: 199}
seconds=94.2
```
== summary.txt
```
HYPER q=5 mode=sampled forms_examined=50000000
squarefree_degree10_forms=38397517
res_shaped=119496
odd_t_hits=0
t_hist={-50: 184, -46: 477, -44: 528, -36: 989, -34: 7507, -32: 2461, -30: 960, -28: 1037, -26: 1444, -24: 4309, -22: 8287, -20: 1435, -18: 3420, -16: 1916, -14: 17066, -12: 2030, -10: 5422, -6: 1971, -4: 10316, -2: 537, 0: 1157, 2: 8590, 4: 1497, 6: 4420, 10: 3041, 12: 1967, 14: 7564, 16: 461, 18: 3713, 20: 1921, 22: 2008, 24: 529, 26: 5424, 32: 2148, 34: 497, 36: 489, 38: 971, 46: 604, 50: 199}
seconds=94.2
```
== hyper\_t\_hist.csv (first 12 of 39 rows)
#table(columns: 2, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*t*], [*count*],
[-50], [184],
[-46], [477],
[-44], [528],
[-36], [989],
[-34], [7507],
[-32], [2461],
[-30], [960],
[-28], [1037],
[-26], [1444],
[-24], [4309],
[-22], [8287],
[-20], [1435]
)

== Graph
#figure(image("../../figures/fig_controls.svg", width: 100%), caption: [Parity of Res-shaped traces: hyperelliptic (Theorem 5, no odd t) and non-hyperelliptic.])
