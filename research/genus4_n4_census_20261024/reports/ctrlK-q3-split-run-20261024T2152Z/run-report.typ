#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: ctrlK-q3-split-run-20261024T2152Z

Descriptive record of one recorded run of examples/genus4\_n4\_census.rs, part CENSUS (exhaustive, q = 3) under
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
run: ctrlK-q3-split 20261024T2152Z
git HEAD: 0fb870d0ee76ba6a1b059e9e34c869518fb412b0
tool: examples/genus4_n4_census.rs sha256 f4c7f7355c838b6f112e09cb412fd3353d087309962c64cd39ad49d79f4c9dad
protocol: research/genus4_n4_census_20261024/PROTOCOL.md sha256 a95b1e35748cefe23a044d8cfd9e059b723cf71fdbc4af56db71257da42c8b33
command: /tmp/claude-0/g4/g4 research/genus4_n4_census_20261024/results/ctrlK-q3-split-run-20261024T2152Z CENSUS 3 split exh 1 0 2000
rustc: rustc 1.97.0 (2d8144b78 2026-07-07)
host: Linux 6.18.44-fc-v153 x86_64, 4 cores, Intel(R) Xeon(R) Processor @ 2.10GHz
threads: 1
wall seconds: 68
```
== tool\_log.txt
```
split q=3: samples=43046721 (exhaustive) res_shaped=179184 allowed_trace_candidates=23040 smooth_prime_order_hits=2304 smooth_fraction_among_prime_cands=0.1000 smooth_fraction_other_res_shaped(sub-sample 2000)=412/2000 secs=67.9
split q=3: total prime-order mass estimate 1.000 +- 0.021 (hit rate per sampled form 5.352e-5)
```
== summary.txt
```
split q=3: samples=43046721 (exhaustive) res_shaped=179184 allowed_trace_candidates=23040 smooth_prime_order_hits=2304 smooth_fraction_among_prime_cands=0.1000 smooth_fraction_other_res_shaped(sub-sample 2000)=412/2000 secs=67.9
split q=3: total prime-order mass estimate 1.000 +- 0.021 (hit rate per sampled form 5.352e-5)
```
== cand\_t\_hist.csv (first 12 of 16 rows)
#table(columns: 3, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*t*], [*candidates*],
[split], [-15], [2304],
[split], [-11], [4608],
[split], [-9], [2304],
[split], [-8], [12672],
[split], [-6], [9216],
[split], [-5], [2304],
[split], [-3], [11520],
[split], [-2], [25344],
[split], [-1], [20736],
[split], [0], [28848],
[split], [1], [28800],
[split], [2], [4608]
)
== hits.csv (first 12 of 23040 rows)
#table(columns: 5, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*code*], [*t*], [*nE*], [*smooth*],
[split], [769526], [-1], [83], [false],
[split], [770270], [-1], [83], [false],
[split], [773192], [-1], [83], [false],
[split], [774323], [11], [71], [true],
[split], [774751], [-1], [83], [false],
[split], [776345], [-1], [83], [false],
[split], [776931], [-1], [83], [false],
[split], [777355], [11], [71], [true],
[split], [777706], [-1], [83], [false],
[split], [779931], [-1], [83], [false],
[split], [780110], [11], [71], [true],
[split], [780696], [-1], [83], [false]
)
== mass\_by\_t.csv (first 1 of 1 rows)
#table(columns: 6, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*t*], [*nE*], [*smooth\_hits*], [*mass\_estimate*], [*mass\_stderr*],
[split], [11], [71], [2304], [1.0000], [0.0208]
)

== Graph
#figure(image("../../figures/fig_mass_by_t.svg", width: 100%), caption: [Mass of hits per prime-order trace with the Haar-density fit.])
