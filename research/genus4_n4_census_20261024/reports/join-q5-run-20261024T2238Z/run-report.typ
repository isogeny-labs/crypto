#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: join-q5-run-20261024T2238Z

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
run: join-q5 20261024T2238Z
git HEAD: 7a579ce24983ef78f6af4ecf3dc9a8c6f10e8a91
tool: examples/genus4_n4_census.rs sha256 efce43f3fdaa848389a03f3d778a9b9604fe23d1d6e5e1bd75e777a49fb6cf01
protocol: research/genus4_n4_census_20261024/PROTOCOL.md sha256 65702f045ec07ff04edc651d7add4725ceecc7cf1bdc79b495a712a8cb942a44
command: /tmp/claude-0/g4/g4 research/genus4_n4_census_20261024/results/join-q5-run-20261024T2238Z JOIN 5 research/genus4_n4_census_20261024/results/cen-q5-cone-run-20261024T2208Z/mass_by_t.csv,research/genus4_n4_census_20261024/results/cen-q5-nonsplit-run-20261024T2210Z/mass_by_t.csv,research/genus4_n4_census_20261024/results/cen-q5-split-run-20261024T2206Z/mass_by_t.csv research/genus4_n4_census_20261024/results/tian-q5-run-20261024T2237Z/tian_by_t.csv
rustc: rustc 1.97.0 (2d8144b78 2026-07-07)
host: Linux 6.18.44-fc-v153 x86_64, 4 cores, Intel(R) Xeon(R) Processor @ 2.10GHz
threads: 1
wall seconds: 0
```
== tool\_log.txt
```
q=5: prime-order traces with an elliptic curve: 14 (ordinary 13); traces with no hit: [49]
Spearman(mass, H) = 0.565
Spearman(mass, Haar density) = 0.775
Spearman(mass, Haar*H/sqrt(disc)) = 0.775
Spearman(mass, #l with Tian kernel (depends = 1/2)) = 0.288
fit mass = c*H: c = 1.317507e0, R^2 (through origin, ratio-of-sums c) = -0.028
fit mass = c*haar: c = 1.985322e2, R^2 (through origin, ratio-of-sums c) = 0.458
fit mass = c*haar*H/sqrt: c = 9.287875e2, R^2 (through origin, ratio-of-sums c) = 0.037
```
== join\_summary.txt
```
q=5: prime-order traces with an elliptic curve: 14 (ordinary 13); traces with no hit: [49]
Spearman(mass, H) = 0.565
Spearman(mass, Haar density) = 0.775
Spearman(mass, Haar*H/sqrt(disc)) = 0.775
Spearman(mass, #l with Tian kernel (depends = 1/2)) = 0.288
fit mass = c*H: c = 1.317507e0, R^2 (through origin, ratio-of-sums c) = -0.028
fit mass = c*haar: c = 1.985322e2, R^2 (through origin, ratio-of-sums c) = 0.458
fit mass = c*haar*H/sqrt: c = 9.287875e2, R^2 (through origin, ratio-of-sums c) = 0.037
```
== join\_by\_t.csv (first 12 of 14 rows)
#table(columns: 12, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*t*], [*nE*], [*ordinary*], [*hits*], [*mass*], [*mass\_se*], [*H*], [*haar\_density*], [*arith\_x\_haar*], [*tian\_yes*], [*tian\_depends*], [*tian\_no*],
[-47], [673], [true], [28], [0.7417], [0.1402], [4.000], [0.014330], [0.003360], [2], [0], [1],
[-33], [659], [true], [1037], [15.9690], [0.5486], [4.000], [0.064274], [0.006844], [3], [0], [0],
[-27], [653], [true], [2036], [36.6702], [0.8797], [8.000], [0.077702], [0.014771], [3], [0], [0],
[-21], [647], [true], [781], [13.6464], [0.5328], [8.000], [0.086747], [0.015294], [2], [0], [1],
[-17], [643], [true], [901], [13.7084], [0.5054], [16.000], [0.090489], [0.030791], [2], [0], [1],
[7], [619], [true], [403], [10.2439], [0.5107], [8.000], [0.080361], [0.012986], [1], [0], [2],
[9], [617], [true], [523], [12.4208], [0.5595], [8.000], [0.077446], [0.012597], [3], [0], [0],
[13], [613], [true], [799], [14.2213], [0.5474], [20.000], [0.070895], [0.029368], [2], [1], [0],
[19], [607], [true], [323], [6.6719], [0.3956], [8.000], [0.059547], [0.010300], [2], [0], [1],
[25], [601], [false], [1070], [22.9753], [0.7374], [12.333], [0.046875], [0.013351], [2], [1], [0],
[27], [599], [true], [637], [9.9517], [0.4385], [8.000], [0.042467], [0.008073], [3], [0], [0],
[33], [593], [true], [81], [1.9807], [0.2201], [4.000], [0.029089], [0.003098], [3], [0], [0]
)

== Graph
#figure(image("../../figures/fig_mass_by_t.svg", width: 100%), caption: [Mass of hits per prime-order trace with the Haar-density fit.])
