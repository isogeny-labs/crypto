#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: cen-q7-nonsplit-run-20261024T2243Z

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
run: cen-q7-nonsplit 20261024T2243Z
git HEAD: 7a579ce24983ef78f6af4ecf3dc9a8c6f10e8a91
tool: examples/genus4_n4_census.rs sha256 efce43f3fdaa848389a03f3d778a9b9604fe23d1d6e5e1bd75e777a49fb6cf01
protocol: research/genus4_n4_census_20261024/PROTOCOL.md sha256 26415a19d8d07de211c7992a6c97ca99a2a3a358fabb5c02e7893639121efd5b
command: /tmp/claude-0/g4/g4 research/genus4_n4_census_20261024/results/cen-q7-nonsplit-run-20261024T2243Z CENSUS 7 nonsplit 30000000 202610247 1 2000
rustc: rustc 1.97.0 (2d8144b78 2026-07-07)
host: Linux 6.18.44-fc-v153 x86_64, 4 cores, Intel(R) Xeon(R) Processor @ 2.10GHz
threads: 1
wall seconds: 522
```
== tool\_log.txt
```
nonsplit 7: 6825623/30000000 samples, res-shaped 216, allowed-trace cand 216, smooth hits 216, 120s
nonsplit 7: 13749106/30000000 samples, res-shaped 415, allowed-trace cand 415, smooth hits 415, 240s
nonsplit 7: 20641572/30000000 samples, res-shaped 591, allowed-trace cand 591, smooth hits 591, 360s
nonsplit 7: 27543164/30000000 samples, res-shaped 781, allowed-trace cand 781, smooth hits 781, 480s
nonsplit q=7: samples=30000000 (random) res_shaped=839 allowed_trace_candidates=839 smooth_prime_order_hits=839 smooth_fraction_among_prime_cands=1.0000 smooth_fraction_other_res_shaped(sub-sample 2000)=0/0 secs=522.0
nonsplit q=7: total prime-order mass estimate 658.599 +- 22.737 (hit rate per sampled form 2.797e-5)
```
== summary.txt
```
nonsplit q=7: samples=30000000 (random) res_shaped=839 allowed_trace_candidates=839 smooth_prime_order_hits=839 smooth_fraction_among_prime_cands=1.0000 smooth_fraction_other_res_shaped(sub-sample 2000)=0/0 secs=522.0
nonsplit q=7: total prime-order mass estimate 658.599 +- 22.737 (hit rate per sampled form 2.797e-5)
```
== cand\_t\_hist.csv (first 12 of 20 rows)
#table(columns: 3, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*t*], [*candidates*],
[nonsplit], [-75], [55],
[nonsplit], [-71], [67],
[nonsplit], [-65], [47],
[nonsplit], [-57], [49],
[nonsplit], [-45], [85],
[nonsplit], [-39], [60],
[nonsplit], [-15], [52],
[nonsplit], [-9], [40],
[nonsplit], [3], [82],
[nonsplit], [9], [43],
[nonsplit], [13], [48],
[nonsplit], [19], [79]
)
== hits.csv (first 12 of 839 rows)
#table(columns: 5, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*code*], [*t*], [*nE*], [*smooth*],
[nonsplit], [32338162100289], [25], [2377], [true],
[nonsplit], [24730639447584], [-9], [2411], [true],
[nonsplit], [12790121929963], [19], [2383], [true],
[nonsplit], [2687952083866], [-71], [2473], [true],
[nonsplit], [7424174273030], [19], [2383], [true],
[nonsplit], [5216633400363], [-15], [2417], [true],
[nonsplit], [13591080077092], [-45], [2447], [true],
[nonsplit], [20627718189362], [19], [2383], [true],
[nonsplit], [9537333093473], [19], [2383], [true],
[nonsplit], [4373706492541], [9], [2393], [true],
[nonsplit], [8268006026927], [-45], [2447], [true],
[nonsplit], [27695046610450], [-45], [2447], [true]
)
== mass\_by\_t.csv (first 12 of 20 rows)
#table(columns: 6, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*t*], [*nE*], [*smooth\_hits*], [*mass\_estimate*], [*mass\_stderr*],
[nonsplit], [-75], [2477], [55], [43.1739], [5.8216],
[nonsplit], [-71], [2473], [67], [52.5937], [6.4253],
[nonsplit], [-65], [2467], [47], [36.8941], [5.3816],
[nonsplit], [-57], [2459], [49], [38.4640], [5.4949],
[nonsplit], [-45], [2447], [85], [66.7233], [7.2372],
[nonsplit], [-39], [2441], [60], [47.0988], [6.0804],
[nonsplit], [-15], [2417], [52], [40.8190], [5.6606],
[nonsplit], [-9], [2411], [40], [31.3992], [4.9647],
[nonsplit], [3], [2399], [82], [64.3684], [7.1083],
[nonsplit], [9], [2393], [43], [33.7542], [5.1475],
[nonsplit], [13], [2389], [48], [37.6791], [5.4385],
[nonsplit], [19], [2383], [79], [62.0135], [6.9771]
)

== Graph
#figure(image("../../figures/fig_mass_by_t.svg", width: 100%), caption: [Mass of hits per prime-order trace with the Haar-density fit.])
