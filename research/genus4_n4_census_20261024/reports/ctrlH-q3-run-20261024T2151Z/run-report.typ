#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: ctrlH-q3-run-20261024T2151Z

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
run: ctrlH-q3 20261024T2151Z
git HEAD: 0fb870d0ee76ba6a1b059e9e34c869518fb412b0
tool: examples/genus4_n4_census.rs sha256 f4c7f7355c838b6f112e09cb412fd3353d087309962c64cd39ad49d79f4c9dad
protocol: research/genus4_n4_census_20261024/PROTOCOL.md sha256 a95b1e35748cefe23a044d8cfd9e059b723cf71fdbc4af56db71257da42c8b33
command: /tmp/claude-0/g4/g4 research/genus4_n4_census_20261024/results/ctrlH-q3-run-20261024T2151Z HYPER 3 exh 1
rustc: rustc 1.97.0 (2d8144b78 2026-07-07)
host: Linux 6.18.44-fc-v153 x86_64, 4 cores, Intel(R) Xeon(R) Processor @ 2.10GHz
threads: 1
wall seconds: 0
```
== tool\_log.txt
```
HYPER q=3 mode=exhaustive forms_examined=177147
squarefree_degree10_forms=104976
res_shaped=1100
odd_t_hits=0
t_hist={-18: 18, -10: 96, -8: 144, -6: 96, -2: 192, 0: 204, 2: 180, 4: 48, 8: 48, 10: 48, 16: 24, 18: 2}
seconds=0.1
```
== summary.txt
```
HYPER q=3 mode=exhaustive forms_examined=177147
squarefree_degree10_forms=104976
res_shaped=1100
odd_t_hits=0
t_hist={-18: 18, -10: 96, -8: 144, -6: 96, -2: 192, 0: 204, 2: 180, 4: 48, 8: 48, 10: 48, 16: 24, 18: 2}
seconds=0.1
```
== hyper\_t\_hist.csv (first 12 of 12 rows)
#table(columns: 2, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*t*], [*count*],
[-18], [18],
[-10], [96],
[-8], [144],
[-6], [96],
[-2], [192],
[0], [204],
[2], [180],
[4], [48],
[8], [48],
[10], [48],
[16], [24],
[18], [2]
)

== Graph
#figure(image("../../figures/fig_controls.svg", width: 100%), caption: [Parity of Res-shaped traces: hyperelliptic (Theorem 5, no odd t) and non-hyperelliptic.])
