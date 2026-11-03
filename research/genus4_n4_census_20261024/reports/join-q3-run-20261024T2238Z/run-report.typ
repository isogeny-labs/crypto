#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: join-q3-run-20261024T2238Z

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
run: join-q3 20261024T2238Z
git HEAD: 7a579ce24983ef78f6af4ecf3dc9a8c6f10e8a91
tool: examples/genus4_n4_census.rs sha256 efce43f3fdaa848389a03f3d778a9b9604fe23d1d6e5e1bd75e777a49fb6cf01
protocol: research/genus4_n4_census_20261024/PROTOCOL.md sha256 65702f045ec07ff04edc651d7add4725ceecc7cf1bdc79b495a712a8cb942a44
command: /tmp/claude-0/g4/g4 research/genus4_n4_census_20261024/results/join-q3-run-20261024T2238Z JOIN 3 research/genus4_n4_census_20261024/results/ctrlK-q3-cone-run-20261024T2151Z/mass_by_t.csv,research/genus4_n4_census_20261024/results/ctrlK-q3-nonsplit-run-20261024T2153Z/mass_by_t.csv,research/genus4_n4_census_20261024/results/ctrlK-q3-split-run-20261024T2152Z/mass_by_t.csv research/genus4_n4_census_20261024/results/tian-q3-run-20261024T2237Z/tian_by_t.csv
rustc: rustc 1.97.0 (2d8144b78 2026-07-07)
host: Linux 6.18.44-fc-v153 x86_64, 4 cores, Intel(R) Xeon(R) Processor @ 2.10GHz
threads: 1
wall seconds: 0
```
== tool\_log.txt
```
q=3: prime-order traces with an elliptic curve: 4 (ordinary 3); traces with no hit: []
Spearman(mass, H) = 0.866
Spearman(mass, Haar density) = 0.500
Spearman(mass, Haar*H/sqrt(disc)) = 1.000
Spearman(mass, #l with Tian kernel (depends = 1/2)) = -0.500
fit mass = c*H: c = 4.615385e-1, R^2 (through origin, ratio-of-sums c) = 0.391
fit mass = c*haar: c = 2.810592e1, R^2 (through origin, ratio-of-sums c) = 0.719
fit mass = c*haar*H/sqrt: c = 1.061910e2, R^2 (through origin, ratio-of-sums c) = 0.975
```
== join\_summary.txt
```
q=3: prime-order traces with an elliptic curve: 4 (ordinary 3); traces with no hit: []
Spearman(mass, H) = 0.866
Spearman(mass, Haar density) = 0.500
Spearman(mass, Haar*H/sqrt(disc)) = 1.000
Spearman(mass, #l with Tian kernel (depends = 1/2)) = -0.500
fit mass = c*H: c = 4.615385e-1, R^2 (through origin, ratio-of-sums c) = 0.391
fit mass = c*haar: c = 2.810592e1, R^2 (through origin, ratio-of-sums c) = 0.719
fit mass = c*haar*H/sqrt: c = 1.061910e2, R^2 (through origin, ratio-of-sums c) = 0.975
```
== join\_by\_t.csv (first 4 of 4 rows)
#table(columns: 12, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*t*], [*nE*], [*ordinary*], [*hits*], [*mass*], [*mass\_se*], [*H*], [*haar\_density*], [*arith\_x\_haar*], [*tian\_yes*], [*tian\_depends*], [*tian\_no*],
[-7], [89], [true], [3888], [3.0000], [0.0481], [5.000], [0.088413], [0.026658], [1], [1], [1],
[-1], [83], [true], [4176], [2.0000], [0.0334], [4.000], [0.090530], [0.020149], [3], [0], [0],
[9], [73], [false], [3024], [2.3333], [0.0424], [4.333], [0.046875], [0.013030], [3], [0], [0],
[11], [71], [true], [2304], [1.0000], [0.0208], [4.000], [0.034535], [0.009696], [2], [0], [1]
)

== Graph
#figure(image("../../figures/fig_mass_by_t.svg", width: 100%), caption: [Mass of hits per prime-order trace with the Haar-density fit.])
