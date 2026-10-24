#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: cen-q5-split-run-20261024T2206Z

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
run: cen-q5-split 20261024T2206Z
git HEAD: 7a579ce24983ef78f6af4ecf3dc9a8c6f10e8a91
tool: examples/genus4_n4_census.rs sha256 cc0bcbe7b428feff4e2acbea3c6887d6abb5031b01f3a49589bd3597b3773dc7
protocol: research/genus4_n4_census_20261024/PROTOCOL.md sha256 457dc80ceded9acbbb0fb39ca1c61e8c3e2bfa773f863100584969c76131187d
command: /tmp/claude-0/g4/g4 research/genus4_n4_census_20261024/results/cen-q5-split-run-20261024T2206Z CENSUS 5 split 50000000 202610245 0 2000
rustc: rustc 1.97.0 (2d8144b78 2026-07-07)
host: Linux 6.18.44-fc-v153 x86_64, 4 cores, Intel(R) Xeon(R) Processor @ 2.10GHz
threads: 1
wall seconds: 108
```
== tool\_log.txt
```
split q=5: samples=50000000 (random) res_shaped=82699 allowed_trace_candidates=2476 smooth_prime_order_hits=2476 smooth_fraction_among_prime_cands=1.0000 smooth_fraction_other_res_shaped(sub-sample 2000)=1109/2000 secs=107.8
split q=5: total prime-order mass estimate 65.592 +- 1.318 (hit rate per sampled form 4.952e-5)
```
== summary.txt
```
split q=5: samples=50000000 (random) res_shaped=82699 allowed_trace_candidates=2476 smooth_prime_order_hits=2476 smooth_fraction_among_prime_cands=1.0000 smooth_fraction_other_res_shaped(sub-sample 2000)=1109/2000 secs=107.8
split q=5: total prime-order mass estimate 65.592 +- 1.318 (hit rate per sampled form 4.952e-5)
```
== cand\_t\_hist.csv (first 12 of 90 rows)
#table(columns: 3, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*t*], [*candidates*],
[split], [-49], [133],
[split], [-48], [86],
[split], [-47], [28],
[split], [-46], [132],
[split], [-45], [110],
[split], [-44], [79],
[split], [-43], [79],
[split], [-42], [99],
[split], [-41], [151],
[split], [-40], [707],
[split], [-39], [449],
[split], [-38], [119]
)
== hits.csv (first 12 of 2476 rows)
#table(columns: 5, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*code*], [*t*], [*nE*], [*smooth*],
[split], [34174412965], [39], [587], [true],
[split], [41046789191], [7], [619], [true],
[split], [4372281092], [9], [617], [true],
[split], [56543983617], [25], [601], [true],
[split], [6772917693], [19], [607], [true],
[split], [72035448669], [9], [617], [true],
[split], [46098340638], [7], [619], [true],
[split], [102118621842], [-27], [653], [true],
[split], [120158365015], [27], [599], [true],
[split], [91157232527], [39], [587], [true],
[split], [102429001849], [19], [607], [true],
[split], [133188596408], [9], [617], [true]
)
== mass\_by\_t.csv (first 12 of 12 rows)
#table(columns: 6, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*type*], [*t*], [*nE*], [*smooth\_hits*], [*mass\_estimate*], [*mass\_stderr*],
[split], [-47], [673], [28], [0.7417], [0.1402],
[split], [-33], [659], [179], [4.7419], [0.3544],
[split], [-27], [653], [319], [8.4506], [0.4731],
[split], [-21], [647], [170], [4.5035], [0.3454],
[split], [-17], [643], [148], [3.9207], [0.3223],
[split], [7], [619], [191], [5.0598], [0.3661],
[split], [9], [617], [394], [10.4374], [0.5258],
[split], [13], [613], [180], [4.7684], [0.3554],
[split], [19], [607], [176], [4.6624], [0.3514],
[split], [25], [601], [432], [11.4441], [0.5506],
[split], [27], [599], [183], [4.8478], [0.3584],
[split], [39], [587], [76], [2.0133], [0.2309]
)

== Graph
#figure(image("../../figures/fig_mass_by_t.svg", width: 100%), caption: [Mass of hits per prime-order trace with the Haar-density fit.])
