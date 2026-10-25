#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: join-q7pre-run-20261024T2239Z

Descriptive record of one recorded run of examples/genus4\_n4\_census.rs, part JOIN under
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
run: join-q7pre 20261024T2239Z
git HEAD: 7a579ce24983ef78f6af4ecf3dc9a8c6f10e8a91
tool: examples/genus4_n4_census.rs sha256 efce43f3fdaa848389a03f3d778a9b9604fe23d1d6e5e1bd75e777a49fb6cf01
protocol: research/genus4_n4_census_20261024/PROTOCOL.md sha256 65702f045ec07ff04edc651d7add4725ceecc7cf1bdc79b495a712a8cb942a44
command: /tmp/claude-0/g4/g4 research/genus4_n4_census_20261024/results/join-q7pre-run-20261024T2239Z JOIN 7 /tmp/claude-0/g4/empty_mass.csv research/genus4_n4_census_20261024/results/tian-q7-run-20261024T2239Z/tian_by_t.csv
rustc: rustc 1.97.0 (2d8144b78 2026-07-07)
host: Linux 6.18.44-fc-v153 x86_64, 4 cores, Intel(R) Xeon(R) Processor @ 2.10GHz
threads: 1
wall seconds: 0
```
== tool\_log.txt
```
q=7: prime-order traces with an elliptic curve: 20 (ordinary 20); traces with no hit: [-75, -71, -65, -57, -45, -39, -15, -9, 3, 9, 13, 19, 25, 31, 45, 51, 55, 61, 69, 93]
Spearman(mass, H) = NaN
Spearman(mass, Haar density) = NaN
Spearman(mass, Haar*H/sqrt(disc)) = NaN
Spearman(mass, #l with Tian kernel (depends = 1/2)) = NaN
fit mass = c*H: c = 0.000000e0, R^2 (through origin, ratio-of-sums c) = NaN
fit mass = c*haar: c = 0.000000e0, R^2 (through origin, ratio-of-sums c) = NaN
fit mass = c*haar*H/sqrt: c = 0.000000e0, R^2 (through origin, ratio-of-sums c) = NaN
```
== join\_summary.txt
```
q=7: prime-order traces with an elliptic curve: 20 (ordinary 20); traces with no hit: [-75, -71, -65, -57, -45, -39, -15, -9, 3, 9, 13, 19, 25, 31, 45, 51, 55, 61, 69, 93]
Spearman(mass, H) = NaN
Spearman(mass, Haar density) = NaN
Spearman(mass, Haar*H/sqrt(disc)) = NaN
Spearman(mass, #l with Tian kernel (depends = 1/2)) = NaN
fit mass = c*H: c = 0.000000e0, R^2 (through origin, ratio-of-sums c) = NaN
fit mass = c*haar: c = 0.000000e0, R^2 (through origin, ratio-of-sums c) = NaN
fit mass = c*haar*H/sqrt: c = 0.000000e0, R^2 (through origin, ratio-of-sums c) = NaN
```
== join\_by\_t.csv (first 12 of 20 rows)
#table(columns: 12, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*t*], [*nE*], [*ordinary*], [*hits*], [*mass*], [*mass\_se*], [*H*], [*haar\_density*], [*arith\_x\_haar*], [*tian\_yes*], [*tian\_depends*], [*tian\_no*],
[-75], [2477], [true], [0], [0.0000], [0.0000], [20.000], [0.048655], [0.015427], [2], [0], [1],
[-71], [2473], [true], [0], [0.0000], [0.0000], [17.333], [0.055147], [0.014151], [3], [0], [0],
[-65], [2467], [true], [0], [0.0000], [0.0000], [24.000], [0.063845], [0.020892], [2], [0], [1],
[-57], [2459], [true], [0], [0.0000], [0.0000], [16.000], [0.073555], [0.014763], [2], [0], [1],
[-45], [2447], [true], [0], [0.0000], [0.0000], [16.000], [0.084258], [0.015485], [2], [0], [1],
[-39], [2441], [true], [0], [0.0000], [0.0000], [16.000], [0.087955], [0.015653], [2], [0], [1],
[-15], [2417], [true], [0], [0.0000], [0.0000], [24.000], [0.092688], [0.022970], [2], [0], [1],
[-9], [2411], [true], [0], [0.0000], [0.0000], [12.000], [0.091579], [0.011261], [2], [0], [1],
[3], [2399], [true], [0], [0.0000], [0.0000], [16.000], [0.086943], [0.014201], [2], [0], [1],
[9], [2393], [true], [0], [0.0000], [0.0000], [12.000], [0.083522], [0.010271], [3], [0], [0],
[13], [2389], [true], [0], [0.0000], [0.0000], [32.000], [0.080869], [0.026642], [2], [0], [1],
[19], [2383], [true], [0], [0.0000], [0.0000], [20.000], [0.076376], [0.015888], [3], [0], [0]
)

== Graph
#figure(image("../../figures/fig_mass_by_t.svg", width: 100%), caption: [Mass of hits per prime-order trace with the Haar-density fit.])
