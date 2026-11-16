#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: cen-q5-nonsplit-run-20261024T2210Z

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
run: cen-q5-nonsplit 20261024T2210Z
git HEAD: 7a579ce24983ef78f6af4ecf3dc9a8c6f10e8a91
tool: examples/genus4_n4_census.rs sha256 cc0bcbe7b428feff4e2acbea3c6887d6abb5031b01f3a49589bd3597b3773dc7
protocol: research/genus4_n4_census_20261024/PROTOCOL.md sha256 457dc80ceded9acbbb0fb39ca1c61e8c3e2bfa773f863100584969c76131187d
command: /tmp/claude-0/g4/g4 research/genus4_n4_census_20261024/results/cen-q5-nonsplit-run-20261024T2210Z CENSUS 5 nonsplit 50000000 202610245 0 2000
rustc: rustc 1.97.0 (2d8144b78 2026-07-07)
host: Linux 6.18.44-fc-v153 x86_64, 4 cores, Intel(R) Xeon(R) Processor @ 2.10GHz
threads: 1
wall seconds: 893
```
== tool\_log.txt
```
nonsplit 5: 6463588/50000000 samples, res-shaped 12302, allowed-trace cand 280, smooth hits 280, 120s
nonsplit 5: 13193329/50000000 samples, res-shaped 25149, allowed-trace cand 585, smooth hits 585, 240s
nonsplit 5: 19960616/50000000 samples, res-shaped 38235, allowed-trace cand 917, smooth hits 917, 360s
nonsplit 5: 26787134/50000000 samples, res-shaped 51318, allowed-trace cand 1252, smooth hits 1252, 480s
nonsplit 5: 33622218/50000000 samples, res-shaped 64347, allowed-trace cand 1539, smooth hits 1539, 600s
nonsplit 5: 40279645/50000000 samples, res-shaped 77305, allowed-trace cand 1854, smooth hits 1854, 720s
nonsplit 5: 47037410/50000000 samples, res-shaped 90330, allowed-trace cand 2158, smooth hits 2158, 840s
nonsplit q=5: samples=50000000 (random) res_shaped=96093 allowed_trace_candidates=2305 smooth_prime_order_hits=2305 smooth_fraction_among_prime_cands=1.0000 smooth_fraction_other_res_shaped(sub-sample 2000)=1067/2000 secs=893.0
nonsplit q=5: total prime-order mass estimate 56.365 +- 1.174 (hit rate per sampled form 4.610e-5)
```
== summary.txt
```
nonsplit q=5: samples=50000000 (random) res_shaped=96093 allowed_trace_candidates=2305 smooth_prime_order_hits=2305 smooth_fraction_among_prime_cands=1.0000 smooth_fraction_other_res_shaped(sub-sample 2000)=1067/2000 secs=893.0
nonsplit q=5: total prime-order mass estimate 56.365 +- 1.174 (hit rate per sampled form 4.610e-5)
```
== cand\_t\_hist.csv (first 12 of 84 rows)
#table(columns: 3, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*t*], [*candidates*],
[nonsplit], [-624], [54],
[nonsplit], [-49], [119],
[nonsplit], [-48], [134],
[nonsplit], [-46], [541],
[nonsplit], [-45], [276],
[nonsplit], [-44], [435],
[nonsplit], [-43], [414],
[nonsplit], [-42], [270],
[nonsplit], [-41], [203],
[nonsplit], [-40], [1266],
[nonsplit], [-39], [1366],
[nonsplit], [-38], [385]
)
== hits.csv (first 12 of 2305 rows)
#table(columns: 5, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*code*], [*t*], [*nE*], [*smooth*],
[nonsplit], [31396477592], [-27], [653], [true],
[nonsplit], [126321324294], [-27], [653], [true],
[nonsplit], [133272213391], [-33], [659], [true],
[nonsplit], [101374332184], [-27], [653], [true],
[nonsplit], [142637028771], [13], [613], [true],
[nonsplit], [37515789705], [-27], [653], [true],
[nonsplit], [45304123164], [-27], [653], [true],
[nonsplit], [146657495888], [33], [593], [true],
[nonsplit], [94124475714], [-27], [653], [true],
[nonsplit], [109986340912], [7], [619], [true],
[nonsplit], [138799106113], [-27], [653], [true],
[nonsplit], [112206438948], [7], [619], [true]
)
== mass\_by\_t.csv (first 12 of 12 rows)
#table(columns: 6, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*t*], [*nE*], [*smooth\_hits*], [*mass\_estimate*], [*mass\_stderr*],
[nonsplit], [-33], [659], [175], [4.2793], [0.3235],
[nonsplit], [-27], [653], [753], [18.4133], [0.6710],
[nonsplit], [-21], [647], [205], [5.0129], [0.3501],
[nonsplit], [-17], [643], [149], [3.6435], [0.2985],
[nonsplit], [7], [619], [212], [5.1841], [0.3560],
[nonsplit], [9], [617], [47], [1.1493], [0.1676],
[nonsplit], [13], [613], [221], [5.4042], [0.3635],
[nonsplit], [19], [607], [36], [0.8803], [0.1467],
[nonsplit], [25], [601], [353], [8.6320], [0.4594],
[nonsplit], [27], [599], [34], [0.8314], [0.1426],
[nonsplit], [33], [593], [81], [1.9807], [0.2201],
[nonsplit], [39], [587], [39], [0.9537], [0.1527]
)

== Graph
#figure(image("../../figures/fig_mass_by_t.svg", width: 100%), caption: [Mass of hits per prime-order trace with the Haar-density fit.])
