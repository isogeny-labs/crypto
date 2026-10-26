#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: ctrlK-q3-cone-run-20261024T2151Z

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
run: ctrlK-q3-cone 20261024T2151Z
git HEAD: 0fb870d0ee76ba6a1b059e9e34c869518fb412b0
tool: examples/genus4_n4_census.rs sha256 f4c7f7355c838b6f112e09cb412fd3353d087309962c64cd39ad49d79f4c9dad
protocol: research/genus4_n4_census_20261024/PROTOCOL.md sha256 a95b1e35748cefe23a044d8cfd9e059b723cf71fdbc4af56db71257da42c8b33
command: /tmp/claude-0/g4/g4 research/genus4_n4_census_20261024/results/ctrlK-q3-cone-run-20261024T2151Z CENSUS 3 cone exh 1 0 2000
rustc: rustc 1.97.0 (2d8144b78 2026-07-07)
host: Linux 6.18.44-fc-v153 x86_64, 4 cores, Intel(R) Xeon(R) Processor @ 2.10GHz
threads: 1
wall seconds: 48
```
== tool\_log.txt
```
cone q=3: samples=14348907 (exhaustive) res_shaped=177363 allowed_trace_candidates=22464 smooth_prime_order_hits=8208 smooth_fraction_among_prime_cands=0.3654 smooth_fraction_other_res_shaped(sub-sample 2000)=955/2000 secs=48.1
cone q=3: total prime-order mass estimate 6.333 +- 0.070 (hit rate per sampled form 5.720e-4)
```
== summary.txt
```
cone q=3: samples=14348907 (exhaustive) res_shaped=177363 allowed_trace_candidates=22464 smooth_prime_order_hits=8208 smooth_fraction_among_prime_cands=0.3654 smooth_fraction_other_res_shaped(sub-sample 2000)=955/2000 secs=48.1
cone q=3: total prime-order mass estimate 6.333 +- 0.070 (hit rate per sampled form 5.720e-4)
```
== cand\_t\_hist.csv (first 12 of 26 rows)
#table(columns: 3, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*t*], [*candidates*],
[cone], [-18], [2160],
[cone], [-17], [648],
[cone], [-16], [1296],
[cone], [-14], [6480],
[cone], [-13], [2592],
[cone], [-12], [1296],
[cone], [-11], [2592],
[cone], [-10], [1296],
[cone], [-9], [5616],
[cone], [-8], [16200],
[cone], [-7], [3888],
[cone], [-6], [2592]
)
== hits.csv (first 12 of 22464 rows)
#table(columns: 5, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*code*], [*t*], [*nE*], [*smooth*],
[cone], [22825], [-1], [83], [false],
[cone], [23152], [-1], [83], [false],
[cone], [23483], [-1], [83], [false],
[cone], [23891], [-1], [83], [false],
[cone], [24310], [-7], [89], [true],
[cone], [24320], [-7], [89], [true],
[cone], [24682], [-1], [83], [false],
[cone], [24764], [-1], [83], [false],
[cone], [25286], [-1], [83], [false],
[cone], [26023], [-1], [83], [false],
[cone], [28615], [-1], [83], [false],
[cone], [29132], [-1], [83], [false]
)
== mass\_by\_t.csv (first 3 of 3 rows)
#table(columns: 6, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*t*], [*nE*], [*smooth\_hits*], [*mass\_estimate*], [*mass\_stderr*],
[cone], [-7], [89], [3888], [3.0000], [0.0481],
[cone], [-1], [83], [1296], [1.0000], [0.0278],
[cone], [9], [73], [3024], [2.3333], [0.0424]
)

== Graph
#figure(image("../../figures/fig_mass_by_t.svg", width: 100%), caption: [Mass of hits per prime-order trace with the Haar-density fit.])
