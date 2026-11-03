#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: cen-q7-split-run-20261024T2239Z

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
run: cen-q7-split 20261024T2239Z
git HEAD: 7a579ce24983ef78f6af4ecf3dc9a8c6f10e8a91
tool: examples/genus4_n4_census.rs sha256 efce43f3fdaa848389a03f3d778a9b9604fe23d1d6e5e1bd75e777a49fb6cf01
protocol: research/genus4_n4_census_20261024/PROTOCOL.md sha256 26415a19d8d07de211c7992a6c97ca99a2a3a358fabb5c02e7893639121efd5b
command: /tmp/claude-0/g4/g4 research/genus4_n4_census_20261024/results/cen-q7-split-run-20261024T2239Z CENSUS 7 split 30000000 202610247 0 2000
rustc: rustc 1.97.0 (2d8144b78 2026-07-07)
host: Linux 6.18.44-fc-v153 x86_64, 4 cores, Intel(R) Xeon(R) Processor @ 2.10GHz
threads: 1
wall seconds: 93
```
== tool\_log.txt
```
split q=7: samples=30000000 (random) res_shaped=18820 allowed_trace_candidates=827 smooth_prime_order_hits=827 smooth_fraction_among_prime_cands=1.0000 smooth_fraction_other_res_shaped(sub-sample 2000)=1356/2000 secs=93.8
split q=7: total prime-order mass estimate 676.228 +- 23.515 (hit rate per sampled form 2.757e-5)
```
== summary.txt
```
split q=7: samples=30000000 (random) res_shaped=18820 allowed_trace_candidates=827 smooth_prime_order_hits=827 smooth_fraction_among_prime_cands=1.0000 smooth_fraction_other_res_shaped(sub-sample 2000)=1356/2000 secs=93.8
split q=7: total prime-order mass estimate 676.228 +- 23.515 (hit rate per sampled form 2.757e-5)
```
== cand\_t\_hist.csv (first 12 of 187 rows)
#table(columns: 3, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*t*], [*candidates*],
[split], [-98], [21],
[split], [-97], [1],
[split], [-96], [4],
[split], [-95], [4],
[split], [-94], [15],
[split], [-93], [1],
[split], [-92], [10],
[split], [-91], [8],
[split], [-90], [23],
[split], [-89], [13],
[split], [-88], [27],
[split], [-87], [6]
)
== hits.csv (first 12 of 827 rows)
#table(columns: 5, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*code*], [*t*], [*nE*], [*smooth*],
[split], [21465600853535], [-45], [2447], [true],
[split], [8256776209770], [69], [2333], [true],
[split], [13932065985012], [55], [2347], [true],
[split], [18817065944093], [-45], [2447], [true],
[split], [9615597712178], [9], [2393], [true],
[split], [22379408582650], [3], [2399], [true],
[split], [12103450054528], [51], [2351], [true],
[split], [8392089724110], [-57], [2459], [true],
[split], [2357605659177], [-45], [2447], [true],
[split], [10798782165558], [45], [2357], [true],
[split], [28838902113595], [9], [2393], [true],
[split], [12688304214622], [61], [2341], [true]
)
== mass\_by\_t.csv (first 12 of 19 rows)
#table(columns: 6, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*t*], [*nE*], [*smooth\_hits*], [*mass\_estimate*], [*mass\_stderr*],
[split], [-75], [2477], [43], [35.1606], [5.3619],
[split], [-71], [2473], [34], [27.8014], [4.7679],
[split], [-65], [2467], [44], [35.9783], [5.4239],
[split], [-57], [2459], [42], [34.3429], [5.2992],
[split], [-45], [2447], [59], [48.2436], [6.2808],
[split], [-39], [2441], [21], [17.1714], [3.7471],
[split], [-15], [2417], [37], [30.2545], [4.9738],
[split], [-9], [2411], [45], [36.7960], [5.4852],
[split], [3], [2399], [68], [55.6028], [6.7428],
[split], [9], [2393], [67], [54.7851], [6.6931],
[split], [13], [2389], [65], [53.1497], [6.5924],
[split], [19], [2383], [82], [67.0504], [7.4045]
)

== Graph
#figure(image("../../figures/fig_mass_by_t.svg", width: 100%), caption: [Mass of hits per prime-order trace with the Haar-density fit.])
