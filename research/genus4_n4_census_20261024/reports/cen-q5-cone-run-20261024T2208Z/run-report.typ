#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: cen-q5-cone-run-20261024T2208Z

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
run: cen-q5-cone 20261024T2208Z
git HEAD: 7a579ce24983ef78f6af4ecf3dc9a8c6f10e8a91
tool: examples/genus4_n4_census.rs sha256 cc0bcbe7b428feff4e2acbea3c6887d6abb5031b01f3a49589bd3597b3773dc7
protocol: research/genus4_n4_census_20261024/PROTOCOL.md sha256 457dc80ceded9acbbb0fb39ca1c61e8c3e2bfa773f863100584969c76131187d
command: /tmp/claude-0/g4/g4 research/genus4_n4_census_20261024/results/cen-q5-cone-run-20261024T2208Z CENSUS 5 cone 50000000 202610245 0 2000
rustc: rustc 1.97.0 (2d8144b78 2026-07-07)
host: Linux 6.18.44-fc-v153 x86_64, 4 cores, Intel(R) Xeon(R) Processor @ 2.10GHz
threads: 1
wall seconds: 136
```
== tool\_log.txt
```
cone 5: 43892004/50000000 samples, res-shaped 148504, allowed-trace cand 3620, smooth hits 3620, 120s
cone q=5: samples=50000000 (random) res_shaped=168992 allowed_trace_candidates=4128 smooth_prime_order_hits=4128 smooth_fraction_among_prime_cands=1.0000 smooth_fraction_other_res_shaped(sub-sample 2000)=1376/2000 secs=136.3
cone q=5: total prime-order mass estimate 41.992 +- 0.654 (hit rate per sampled form 8.256e-5)
```
== summary.txt
```
cone q=5: samples=50000000 (random) res_shaped=168992 allowed_trace_candidates=4128 smooth_prime_order_hits=4128 smooth_fraction_among_prime_cands=1.0000 smooth_fraction_other_res_shaped(sub-sample 2000)=1376/2000 secs=136.3
cone q=5: total prime-order mass estimate 41.992 +- 0.654 (hit rate per sampled form 8.256e-5)
```
== cand\_t\_hist.csv (first 12 of 85 rows)
#table(columns: 3, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*t*], [*candidates*],
[cone], [-50], [522],
[cone], [-49], [231],
[cone], [-46], [1118],
[cone], [-45], [465],
[cone], [-44], [184],
[cone], [-43], [388],
[cone], [-41], [268],
[cone], [-39], [691],
[cone], [-38], [213],
[cone], [-37], [1431],
[cone], [-36], [91],
[cone], [-35], [109]
)
== hits.csv (first 12 of 4128 rows)
#table(columns: 5, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*code*], [*t*], [*nE*], [*smooth*],
[cone], [9985578608], [-17], [643], [true],
[cone], [4176399056], [-21], [647], [true],
[cone], [10209046869], [-27], [653], [true],
[cone], [27730488661], [-33], [659], [true],
[cone], [2615248492], [-21], [647], [true],
[cone], [17160923437], [27], [599], [true],
[cone], [3637424030], [9], [617], [true],
[cone], [12136877657], [-33], [659], [true],
[cone], [21965452193], [-27], [653], [true],
[cone], [15189766298], [-17], [643], [true],
[cone], [18032064696], [27], [599], [true],
[cone], [11154693818], [25], [601], [true]
)
== mass\_by\_t.csv (first 10 of 10 rows)
#table(columns: 6, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*t*], [*nE*], [*smooth\_hits*], [*mass\_estimate*], [*mass\_stderr*],
[cone], [-33], [659], [683], [6.9478], [0.2659],
[cone], [-27], [653], [964], [9.8063], [0.3158],
[cone], [-21], [647], [406], [4.1300], [0.2050],
[cone], [-17], [643], [604], [6.1442], [0.2500],
[cone], [9], [617], [82], [0.8341], [0.0921],
[cone], [13], [613], [398], [4.0487], [0.2029],
[cone], [19], [607], [111], [1.1292], [0.1072],
[cone], [25], [601], [285], [2.8992], [0.1717],
[cone], [27], [599], [420], [4.2725], [0.2085],
[cone], [39], [587], [175], [1.7802], [0.1346]
)

== Graph
#figure(image("../../figures/fig_mass_by_t.svg", width: 100%), caption: [Mass of hits per prime-order trace with the Haar-density fit.])
