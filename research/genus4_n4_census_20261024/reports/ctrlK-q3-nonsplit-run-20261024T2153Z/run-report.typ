#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: ctrlK-q3-nonsplit-run-20261024T2153Z

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
run: ctrlK-q3-nonsplit 20261024T2153Z
git HEAD: 0fb870d0ee76ba6a1b059e9e34c869518fb412b0
tool: examples/genus4_n4_census.rs sha256 f4c7f7355c838b6f112e09cb412fd3353d087309962c64cd39ad49d79f4c9dad
protocol: research/genus4_n4_census_20261024/PROTOCOL.md sha256 a95b1e35748cefe23a044d8cfd9e059b723cf71fdbc4af56db71257da42c8b33
command: /tmp/claude-0/g4/g4 research/genus4_n4_census_20261024/results/ctrlK-q3-nonsplit-run-20261024T2153Z CENSUS 3 nonsplit exh 1 0 2000
rustc: rustc 1.97.0 (2d8144b78 2026-07-07)
host: Linux 6.18.44-fc-v153 x86_64, 4 cores, Intel(R) Xeon(R) Processor @ 2.10GHz
threads: 1
wall seconds: 196
```
== tool\_log.txt
```
nonsplit 3: 26891264/43046721 samples, res-shaped 170092, allowed-trace cand 19023, smooth hits 1660, 120s
nonsplit q=3: samples=43046721 (exhaustive) res_shaped=274380 allowed_trace_candidates=31680 smooth_prime_order_hits=2880 smooth_fraction_among_prime_cands=0.0909 smooth_fraction_other_res_shaped(sub-sample 2000)=196/2000 secs=195.7
nonsplit q=3: total prime-order mass estimate 1.000 +- 0.019 (hit rate per sampled form 6.690e-5)
```
== summary.txt
```
nonsplit q=3: samples=43046721 (exhaustive) res_shaped=274380 allowed_trace_candidates=31680 smooth_prime_order_hits=2880 smooth_fraction_among_prime_cands=0.0909 smooth_fraction_other_res_shaped(sub-sample 2000)=196/2000 secs=195.7
nonsplit q=3: total prime-order mass estimate 1.000 +- 0.019 (hit rate per sampled form 6.690e-5)
```
== cand\_t\_hist.csv (first 12 of 18 rows)
#table(columns: 3, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*t*], [*candidates*],
[nonsplit], [-80], [2160],
[nonsplit], [-17], [1440],
[nonsplit], [-16], [5760],
[nonsplit], [-14], [5760],
[nonsplit], [-13], [11520],
[nonsplit], [-12], [2880],
[nonsplit], [-11], [8640],
[nonsplit], [-10], [5760],
[nonsplit], [-9], [14400],
[nonsplit], [-8], [50400],
[nonsplit], [-6], [14400],
[nonsplit], [-5], [8640]
)
== hits.csv (first 12 of 31680 rows)
#table(columns: 5, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*code*], [*t*], [*nE*], [*smooth*],
[nonsplit], [10196], [-1], [83], [false],
[nonsplit], [10660], [-1], [83], [false],
[nonsplit], [12140], [-1], [83], [false],
[nonsplit], [13090], [-1], [83], [false],
[nonsplit], [16433], [-1], [83], [false],
[nonsplit], [17383], [-1], [83], [false],
[nonsplit], [18863], [-1], [83], [false],
[nonsplit], [19327], [-1], [83], [false],
[nonsplit], [23053], [-1], [83], [false],
[nonsplit], [23723], [-1], [83], [false],
[nonsplit], [25159], [-1], [83], [false],
[nonsplit], [25991], [-1], [83], [false]
)
== mass\_by\_t.csv (first 1 of 1 rows)
#table(columns: 6, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*t*], [*nE*], [*smooth\_hits*], [*mass\_estimate*], [*mass\_stderr*],
[nonsplit], [-1], [83], [2880], [1.0000], [0.0186]
)

== Graph
#figure(image("../../figures/fig_mass_by_t.svg", width: 100%), caption: [Mass of hits per prime-order trace with the Haar-density fit.])
