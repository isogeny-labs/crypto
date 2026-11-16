#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: join-q7-run-20261024T2258Z

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
run: join-q7 20261024T2258Z
git HEAD: b08305739b2f95bb83966c2830da5e8d14eb2d92
tool: examples/genus4_n4_census.rs sha256 efce43f3fdaa848389a03f3d778a9b9604fe23d1d6e5e1bd75e777a49fb6cf01
protocol: research/genus4_n4_census_20261024/PROTOCOL.md sha256 26415a19d8d07de211c7992a6c97ca99a2a3a358fabb5c02e7893639121efd5b
command: /tmp/claude-0/g4/g4 research/genus4_n4_census_20261024/results/join-q7-run-20261024T2258Z JOIN 7 research/genus4_n4_census_20261024/results/cen-q7-cone-run-20261024T2241Z/mass_by_t.csv,research/genus4_n4_census_20261024/results/cen-q7-nonsplit-run-20261024T2243Z/mass_by_t.csv,research/genus4_n4_census_20261024/results/cen-q7-split-run-20261024T2239Z/mass_by_t.csv research/genus4_n4_census_20261024/results/tian-q7-run-20261024T2239Z/tian_by_t.csv
rustc: rustc 1.97.0 (2d8144b78 2026-07-07)
host: Linux 6.18.44-fc-v153 x86_64, 4 cores, Intel(R) Xeon(R) Processor @ 2.10GHz
threads: 1
wall seconds: 0
```
== tool\_log.txt
```
q=7: prime-order traces with an elliptic curve: 20 (ordinary 20); traces with no hit: []
Spearman(mass, H) = 0.386
Spearman(mass, Haar density) = 0.636
Spearman(mass, Haar*H/sqrt(disc)) = 0.579
Spearman(mass, #l with Tian kernel (depends = 1/2)) = 0.217
fit mass = c*H: c = 4.295237e0, R^2 (through origin, ratio-of-sums c) = -0.023
fit mass = c*haar: c = 1.233694e3, R^2 (through origin, ratio-of-sums c) = 0.556
fit mass = c*haar*H/sqrt: c = 5.690572e3, R^2 (through origin, ratio-of-sums c) = 0.338
```
== join\_summary.txt
```
q=7: prime-order traces with an elliptic curve: 20 (ordinary 20); traces with no hit: []
Spearman(mass, H) = 0.386
Spearman(mass, Haar density) = 0.636
Spearman(mass, Haar*H/sqrt(disc)) = 0.579
Spearman(mass, #l with Tian kernel (depends = 1/2)) = 0.217
fit mass = c*H: c = 4.295237e0, R^2 (through origin, ratio-of-sums c) = -0.023
fit mass = c*haar: c = 1.233694e3, R^2 (through origin, ratio-of-sums c) = 0.556
fit mass = c*haar*H/sqrt: c = 5.690572e3, R^2 (through origin, ratio-of-sums c) = 0.338
```
== join\_by\_t.csv (first 12 of 20 rows)
#table(columns: 12, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*t*], [*nE*], [*ordinary*], [*hits*], [*mass*], [*mass\_se*], [*H*], [*haar\_density*], [*arith\_x\_haar*], [*tian\_yes*], [*tian\_depends*], [*tian\_no*],
[-75], [2477], [true], [164], [93.4391], [8.1300], [20.000], [0.048655], [0.015427], [2], [0], [1],
[-71], [2473], [true], [143], [90.0071], [8.1374], [17.333], [0.055147], [0.014151], [3], [0], [0],
[-65], [2467], [true], [111], [77.4495], [7.7089], [24.000], [0.063845], [0.020892], [2], [0], [1],
[-57], [2459], [true], [159], [88.3692], [7.8636], [16.000], [0.073555], [0.014763], [2], [0], [1],
[-45], [2447], [true], [230], [134.6486], [9.8148], [16.000], [0.084258], [0.015485], [2], [0], [1],
[-39], [2441], [true], [108], [70.4493], [7.2406], [16.000], [0.087955], [0.015653], [2], [0], [1],
[-15], [2417], [true], [165], [88.4667], [7.7950], [24.000], [0.092688], [0.022970], [2], [0], [1],
[-9], [2411], [true], [123], [76.8918], [7.5317], [12.000], [0.091579], [0.011261], [2], [0], [1],
[3], [2399], [true], [228], [137.8221], [10.0039], [16.000], [0.086943], [0.014201], [2], [0], [1],
[9], [2393], [true], [181], [104.7882], [8.6610], [12.000], [0.083522], [0.010271], [3], [0], [0],
[13], [2389], [true], [191], [108.6797], [8.7819], [32.000], [0.080869], [0.026642], [2], [0], [1],
[19], [2383], [true], [264], [152.6362], [10.4356], [20.000], [0.076376], [0.015888], [3], [0], [0]
)

== Graph
#figure(image("../../figures/fig_mass_by_t.svg", width: 100%), caption: [Mass of hits per prime-order trace with the Haar-density fit.])
