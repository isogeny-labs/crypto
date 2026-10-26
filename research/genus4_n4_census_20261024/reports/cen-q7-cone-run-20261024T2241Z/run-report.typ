#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: cen-q7-cone-run-20261024T2241Z

Descriptive record of one recorded run of examples/genus4\_n4\_census.rs, part CENSUS (sampled) under
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
run: cen-q7-cone 20261024T2241Z
git HEAD: 7a579ce24983ef78f6af4ecf3dc9a8c6f10e8a91
tool: examples/genus4_n4_census.rs sha256 efce43f3fdaa848389a03f3d778a9b9604fe23d1d6e5e1bd75e777a49fb6cf01
protocol: research/genus4_n4_census_20261024/PROTOCOL.md sha256 26415a19d8d07de211c7992a6c97ca99a2a3a358fabb5c02e7893639121efd5b
command: /tmp/claude-0/g4/g4 research/genus4_n4_census_20261024/results/cen-q7-cone-run-20261024T2241Z CENSUS 7 cone 30000000 202610247 0 2000
rustc: rustc 1.97.0 (2d8144b78 2026-07-07)
host: Linux 6.18.44-fc-v153 x86_64, 4 cores, Intel(R) Xeon(R) Processor @ 2.10GHz
threads: 1
wall seconds: 116
```
== tool\_log.txt
```
cone q=7: samples=30000000 (random) res_shaped=34702 allowed_trace_candidates=949 smooth_prime_order_hits=949 smooth_fraction_among_prime_cands=1.0000 smooth_fraction_other_res_shaped(sub-sample 2000)=1516/2000 secs=115.2
cone q=7: total prime-order mass estimate 217.186 +- 7.050 (hit rate per sampled form 3.163e-5)
```
== summary.txt
```
cone q=7: samples=30000000 (random) res_shaped=34702 allowed_trace_candidates=949 smooth_prime_order_hits=949 smooth_fraction_among_prime_cands=1.0000 smooth_fraction_other_res_shaped(sub-sample 2000)=1516/2000 secs=115.2
cone q=7: total prime-order mass estimate 217.186 +- 7.050 (hit rate per sampled form 3.163e-5)
```
== cand\_t\_hist.csv (first 12 of 177 rows)
#table(columns: 3, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*t*], [*candidates*],
[cone], [-98], [84],
[cone], [-95], [8],
[cone], [-94], [48],
[cone], [-93], [7],
[cone], [-91], [15],
[cone], [-90], [53],
[cone], [-89], [44],
[cone], [-88], [44],
[cone], [-87], [18],
[cone], [-86], [110],
[cone], [-84], [52],
[cone], [-83], [52]
)
== hits.csv (first 12 of 949 rows)
#table(columns: 5, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*code*], [*t*], [*nE*], [*smooth*],
[cone], [1623141785906], [13], [2389], [true],
[cone], [1935115740542], [25], [2377], [true],
[cone], [1654331499937], [19], [2383], [true],
[cone], [2676523467197], [-9], [2411], [true],
[cone], [3202755716986], [-45], [2447], [true],
[cone], [391616815344], [-45], [2447], [true],
[cone], [2083491091311], [31], [2371], [true],
[cone], [2875489973986], [25], [2377], [true],
[cone], [1785212176126], [-15], [2417], [true],
[cone], [2433100403872], [-45], [2447], [true],
[cone], [4326092431570], [9], [2393], [true],
[cone], [1182928668445], [3], [2399], [true]
)
== mass\_by\_t.csv (first 12 of 19 rows)
#table(columns: 6, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*t*], [*nE*], [*smooth\_hits*], [*mass\_estimate*], [*mass\_stderr*],
[cone], [-75], [2477], [66], [15.1046], [1.8592],
[cone], [-71], [2473], [42], [9.6120], [1.4832],
[cone], [-65], [2467], [20], [4.5771], [1.0235],
[cone], [-57], [2459], [68], [15.5623], [1.8872],
[cone], [-45], [2447], [86], [19.6817], [2.1223],
[cone], [-39], [2441], [27], [6.1791], [1.1892],
[cone], [-15], [2417], [76], [17.3932], [1.9951],
[cone], [-9], [2411], [38], [8.6966], [1.4108],
[cone], [3], [2399], [78], [17.8509], [2.0212],
[cone], [9], [2393], [71], [16.2489], [1.9284],
[cone], [13], [2389], [78], [17.8509], [2.0212],
[cone], [19], [2383], [103], [23.5723], [2.3226]
)

== Graph
#figure(image("../../figures/fig_mass_by_t.svg", width: 100%), caption: [Mass of hits per prime-order trace with the Haar-density fit.])
