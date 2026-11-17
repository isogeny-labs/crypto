#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: ctrlH-q7-run-20261024T2158Z

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
run: ctrlH-q7 20261024T2158Z
git HEAD: 0fb870d0ee76ba6a1b059e9e34c869518fb412b0
tool: examples/genus4_n4_census.rs sha256 f4c7f7355c838b6f112e09cb412fd3353d087309962c64cd39ad49d79f4c9dad
protocol: research/genus4_n4_census_20261024/PROTOCOL.md sha256 a95b1e35748cefe23a044d8cfd9e059b723cf71fdbc4af56db71257da42c8b33
command: /tmp/claude-0/g4/g4 research/genus4_n4_census_20261024/results/ctrlH-q7-run-20261024T2158Z HYPER 7 50000000 202610247
rustc: rustc 1.97.0 (2d8144b78 2026-07-07)
host: Linux 6.18.44-fc-v153 x86_64, 4 cores, Intel(R) Xeon(R) Processor @ 2.10GHz
threads: 1
wall seconds: 143
```
== tool\_log.txt
```
HYPER q=7 mode=sampled forms_examined=50000000
squarefree_degree10_forms=41985520
res_shaped=55040
odd_t_hits=0
t_hist={-98: 252, -96: 101, -94: 156, -90: 167, -88: 88, -86: 265, -84: 98, -82: 744, -80: 100, -78: 633, -76: 163, -74: 210, -70: 323, -68: 684, -66: 441, -64: 889, -62: 1729, -60: 313, -58: 644, -56: 384, -54: 564, -52: 350, -50: 193, -48: 513, -46: 3594, -44: 154, -42: 894, -40: 378, -38: 1560, -36: 202, -34: 644, -32: 186, -30: 3073, -26: 1428, -24: 173, -22: 713, -20: 925, -18: 1451, -16: 354, -12: 503, -10: 767, -8: 649, -6: 579, -4: 50, -2: 1524, 0: 631, 2: 3652, 4: 265, 6: 544, 8: 351, 10: 954, 12: 593, 14: 622, 16: 255, 18: 1914, 20: 298, 22: 1456, 24: 400, 26: 1273, 28: 183, 30: 966, 32: 530, 34: 3069, 36: 57, 38: 165, 40: 284, 42: 374, 44: 153, 46: 981, 48: 427, 50: 733, 52: 392, 54: 145, 56: 42, 58: 618, 60: 110, 62: 839, 64: 228, 66: 551, 68: 138, 72: 147, 74: 53, 76: 49, 80: 348, 82: 342, 90: 46, 94: 57}
seconds=143.4
```
== summary.txt
```
HYPER q=7 mode=sampled forms_examined=50000000
squarefree_degree10_forms=41985520
res_shaped=55040
odd_t_hits=0
t_hist={-98: 252, -96: 101, -94: 156, -90: 167, -88: 88, -86: 265, -84: 98, -82: 744, -80: 100, -78: 633, -76: 163, -74: 210, -70: 323, -68: 684, -66: 441, -64: 889, -62: 1729, -60: 313, -58: 644, -56: 384, -54: 564, -52: 350, -50: 193, -48: 513, -46: 3594, -44: 154, -42: 894, -40: 378, -38: 1560, -36: 202, -34: 644, -32: 186, -30: 3073, -26: 1428, -24: 173, -22: 713, -20: 925, -18: 1451, -16: 354, -12: 503, -10: 767, -8: 649, -6: 579, -4: 50, -2: 1524, 0: 631, 2: 3652, 4: 265, 6: 544, 8: 351, 10: 954, 12: 593, 14: 622, 16: 255, 18: 1914, 20: 298, 22: 1456, 24: 400, 26: 1273, 28: 183, 30: 966, 32: 530, 34: 3069, 36: 57, 38: 165, 40: 284, 42: 374, 44: 153, 46: 981, 48: 427, 50: 733, 52: 392, 54: 145, 56: 42, 58: 618, 60: 110, 62: 839, 64: 228, 66: 551, 68: 138, 72: 147, 74: 53, 76: 49, 80: 348, 82: 342, 90: 46, 94: 57}
seconds=143.4
```
== hyper\_t\_hist.csv (first 12 of 87 rows)
#table(columns: 2, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*t*], [*count*],
[-98], [252],
[-96], [101],
[-94], [156],
[-90], [167],
[-88], [88],
[-86], [265],
[-84], [98],
[-82], [744],
[-80], [100],
[-78], [633],
[-76], [163],
[-74], [210]
)

== Graph
#figure(image("../../figures/fig_controls.svg", width: 100%), caption: [Parity of Res-shaped traces: hyperelliptic (Theorem 5, no odd t) and non-hyperelliptic.])
