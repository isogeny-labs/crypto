#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set table(inset: 4pt, stroke: 0.4pt + gray)
#show table: set text(size: 7.5pt)
= Exponent study: does the subgroup-base algebraic route have an
exponent lever? (2026-11-10)
<exponent-study-does-the-subgroup-base-algebraic-route-have-an-exponent-lever-2026-11-10>
#strong[Class.] An engine measurement in counted units: F\_p
multiplications of the F4 row reduction, `F4Report.field_ops`, from
`curve25519_family f4sub`. It measures the cost of one decomposition
system against the base size. It is not an end-to-end solve, not an
ecbench session, and not a leaderboard or `benchmarks/` row. No pcc
claim was registered, so every measured number below is an unverified
assertion. Rho remains ahead on every track \[pcc:unverified\].

#strong[Question (owner preregistration, 2026-11-08).] The question
concerns Algorithm 1 of the algebraic ECDLP attack on prime curves
(Petit, Kosters, Messeng, PKC 2016) and the cost model of its section
3.2. The total cost is about P(p,m) + (m! p / (deg L)^(m-1)) T(E,m,L) +
(deg L)^omega, with deg L \= |V|. With omega \= 2 and |V| \= p^a, m \= 3
balances at a \= 1/4 and m \= 4 at a \= 1/5. An exponent lever exists at
m \= 4 only if T(E,4,L) grows more slowly than |V|^(1/2). At m \= 3 it
exists only if T stays flat in |V|. This study fits the exponent s of T
against |V|. It uses the symmetrised system over multiplicative-subgroup
bases of the Curve25519-family miniatures.

#strong[Answer.] No lever was found at m \= 2 \[pcc:unverified\]. At m
\= 3 and m \= 4 the frozen resource limits censored the series before
the preregistered estimator could apply, so the verdict there is
\"undetermined\" \[pcc:unverified\]. The limits were 3.5 GB of virtual
memory per process (`ulimit -v`) and 1,200 s of wall time per target, on
a shared 4-core host. This is a resource limit, not evidence that no
lever exists at larger |V|.

- #strong[m \= 2.] The preregistered fit gives s \= 2.77 \[2.31, 3.23\]
  at k \= 20, 2.81 \[1.81, 3.80\] at k \= 22 and 2.57 \[0.91, 4.24\] at
  k \= 24 \[pcc:unverified\].
- #strong[m \= 3 and m \= 4: the preregistered estimator could not be
  applied.] Before the first censored cell, the complete cells with d
  \>\= 9 span 3.0x in |V| at k \= 20 and 3.7x at k \= 22, below the
  required 4x. At m \= 4 only one such cell is complete. The
  preregistered verdict is therefore \"undetermined\" for both
  \[pcc:unverified\].
- #strong[Other readings.] The m \= 3 sensitivity fits add the boundary
  cells d \< 9 (d \= 3 and 7 at k \= 20, d \= 3 and 5 at k \= 22). They
  give 6.44 \[3.49, 9.39\] at k \= 20 and 5.76 \[3.77, 7.75\] at k \=
  22, and they are the only m \= 3 readings whose lower bounds lie above
  0 \[pcc:unverified\]. The d \>\= 9 fits, computed despite the span
  gate, give 5.56 \[-8.75, 19.88\] at k \= 20 (4 cells) and 4.51
  \[-1.27, 10.29\] at k \= 22 (6 cells), so the d \>\= 9 cells alone do
  not exclude s \< 0 at 95% at m \= 3 \[pcc:unverified\]. At m \= 4 the
  only fit, over the three cells d \= 3, 7 and 9, gives 9.86 \[-59.9,
  79.6\] \[pcc:unverified\]. Every point estimate lies far above the m
  \= 3 threshold (s \< 0) and the m \= 4 threshold (s \< 0.5), and no
  upper bound lies below them, so no reading meets the lever rule
  \[pcc:unverified\]. From |V| \= 3 to |V| \= 9, the m \= 4 F4 cost T
  rises 3.4e5-fold; T excludes the per-target construction of the
  symmetrised system (section 8) \[pcc:unverified\].

#strong[Protocol and companion files.]
#link("PROTOCOL.md")[`PROTOCOL.md`] was frozen on 2026-11-08, before any
measurement. Its sha256 is in
#link("data/frozen.sha256")[`data/frozen.sha256`]. On 2026-11-10 the
hashes of the protocol, `scripts/run_grid.py`, `scripts/analyse.py` and
the engine binary were recomputed, and all four equal the frozen values
\[pcc:unverified\]. #link("PRIOR_STUDIES.md")[`PRIOR_STUDIES.md`] is an
addendum written after the grid had run. The report is
#link("REPORT.pdf")[`REPORT.pdf`] and the figure is
#link("exponent_fit.svg")[`exponent_fit.svg`]. The independent
validation is #link("validator-report.json")[`validator-report.json`];
its errata are applied in the section \"Validation\" at the end.

#strong[Post-data change to a non-frozen script.] On 2026-11-10, after
all data were taken, the non-frozen `scripts/tables.py` was edited in
commit `8c544a1829`. The edit changes only the status label of a
censored cell in `data/tables.md`, which now names the cause (\"1200 s
cap\" or \"OOM at 3.5 GB\") instead of always \"OOM\". No number, fit or
verdict depends on it, and the frozen `scripts/analyse.py` was not
touched.

#strong[Source, toolchain and binary.] The engine binary was built from
the tree at commit `8d0388b10a` (the protocol freeze,
2026-11-08T23:50:48Z), before the merge `51b1ba83bc`. A `cargo build` at
a later HEAD gives a different binary. The toolchain on the host is
`rustc 1.97.0 (2d8144b78 2026-07-07)` (`cargo 1.97.0`), read on
2026-11-10; the build itself did not record its toolchain. The frozen
binary, sha256
`dc2c30a4242d80281622b2cb42cbaf9cb457d041ffbc9a4d35794bb893bf3faf`, was
copied out of the shared build directory to
`/home/user/bin-archive/curve25519_family-dc2c30a4` on 2026-11-10, and
the copy has the same sha256. It is not committed to Git and not yet in
the artifact store.

#strong[Resume after a container restart (2026-11-10).] A container
restart killed the grid runner while it was measuring
`k20_m3_sym_d27_g0`. The runner writes a cell file only when all six
targets of the cell are done, so no partial data from that cell exists
or was used. All other cell files were complete or censored before the
restart, and none was redone. The interrupted cell was rerun from
scratch with the frozen binary. The binary was a copy of
`/home/user/ctarget-f1/release/curve25519_family`, with sha256
`dc2c30a4242d80281622b2cb42cbaf9cb457d041ffbc9a4d35794bb893bf3faf`
(equal to the frozen hash). The binary is newer than
`subgroup_semaev.rs`, `pkm_semaev.rs` and
`src/bin/curve25519_family.rs`, so it was not rebuilt. The rerun
completed: 6 of 6 targets, each under the 1200 s cap. Under the chain
rule the runner then ran d \= 57, whose first target hit the 1200 s cap,
so that cell is censored. The k \= 22 m \= 3 cell at d \= 55 had ended
in an out-of-memory exit before the restart (`data/run.log`)
\[pcc:unverified\].

#strong[Host, load and wall time.] The runner logged no host name, core
count or concurrent load. The censoring rule is wall clock (monotonic)
on a shared 4-core host. Per the session log, the d \= 27 rerun and the
censored d \= 57 target ran while another agent built `ecbench` on the
same host. The d \= 57 target ran at about 6.9e7 field ops/s against
about 2.9e8 for the d \= 27 rerun, so its censoring may partly reflect
host load \[pcc:unverified\]. There are also unexplained wall-clock
gaps: `k22_m3_sym_d55` was written about 26.5 h after `d41`, and the
original d \= 27 attempt ran about 27 h without writing a cell file,
against about 1.5 h for the rerun \[pcc:unverified\]. These fit a VM
pause that the monotonic budget did not see, but that cannot be checked.
Wall time is not a claimed quantity of this study; only `field_ops` is.

== 1. System and engine
<1-system-and-engine>
- #strong[Module.]
  #link("../../src/cryptanalysis/ic_framework/subgroup_semaev.rs")[`src/cryptanalysis/ic_framework/subgroup_semaev.rs`],
  commands `curve25519_family f4sub` and `subcensus`. It is a separate
  module rather than a new `Membership` variant of `pkm_semaev.rs`: the
  tower engine\'s bit-mask monomials hold exponents 0 or 1 only, so they
  cannot hold the odd-degree chain rules y^3 \= y\'. No path of
  `pkm_semaev` or `f4_fp_tower` changed.
- #strong[Base.] V \= lo + g mu\_d, d | p - 1 odd, d \= e\_0 e\_1 ...
  e\_{L-1} (primes ascending, with multiplicity), lo \= A/3 (the
  Montgomery frame), g \= w^gidx with w the smallest primitive root.
- #strong[Per-summand constraint.] This is the power-map chain of the
  paper: x\_i \= lo + g y\_{i,0}, y\_{i,j}^{e\_j} \= y\_{i,j+1} for j \<
  L - 1, and the closing relation y\_{i,L-1}^{e\_{L-1}} \= 1. The
  paper\'s closing map is 1 - x^{p\_n\'}; this is the same relation.
- #strong[Target and form.] The target X \= x(aG) is a constant, with a
  uniform in \[1, r - 1\]. The form `sym` (primary) adds e\_k -
  sigma\_k(x\_1..x\_m) for k \= 1..m and the Semaev polynomial
  S\_{m+1}(x\_1..x\_m, X), written in e\_1..e\_m by interpolation. The
  interpolation is checked on 12 extra points. The form `plain`
  substitutes sigma\_k(x) for e\_k (ablation).
- #strong[Engine.] `f4_fp::f4`, grevlex, max degree 400. There is no
  guessing (G \= 0): T is the cost of one complete system.
- #strong[Unit tests.] The module has six tests. They cover: the
  summation polynomials vanish on sums (m \= 2, 3, 4); S~ equals S (m \=
  2, 3, 4); roots of unity and the chain factorisation
  (`roots_of_unity_and_factors`); the systems on p \= 211 (m \= 2 with d
  \= 7 and 15, m \= 3 with d \= 7) are consistent if and only if
  S\_{m+1} has a root in V^m, and a sum of base points always gives a
  consistent system; the bit-interval base matches brute force. The
  tests do not assert that a system is consistent exactly when the
  target is a sum of m base points; for m \>\= 3 twist points can also
  give roots. No m \= 4 consistency test exists. The producer ran them
  from the release test binary built at the module\'s commit (6 passed,
  0 failed) \[pcc:unverified\]. The test log was not committed, and the
  tests were not rerun for this revision, because building was not
  allowed on the host.

== 2. Grid and census
<2-grid-and-census>
The grid is as frozen in section 4 of the protocol:

- 6 targets per cell, with seed 20261108 + 1000 m + d;
- a cap of 1200 s per target and 3.5 GB of memory per process;
- each (k, m, form) chain runs d ascending and stops at its first
  censored cell. Larger d are not attempted.

Every chain in the protocol has now ended, at its last d or at a
censored cell. The census (`data/census.json`, from `scripts/census.py`)
is an exact enumeration \[pcc:unverified\]. Its columns are defined as
follows:

- #strong[columns:] the number of abscissae of V on the curve;
- #strong[base points:] the number of signed base points;
- #strong[covered:] the number of distinct points of the order-r
  subgroup that are sums of exactly m base points.

Almost every target lies outside the covered set, so almost every system
is inconsistent, as in the real attack: 8 of the 414 measured systems
are consistent \[pcc:unverified\]. At m \= 3 the covered set is empty
for d \= 3 and 9 at k \= 20 and for d \= 3 at k \= 22
\[pcc:unverified\]. T is therefore mostly the cost of proving that a
system has no solution \[pcc:unverified\].

The four curves are measured from `data/census.json`:

#align(center)[#table(
  columns: 3,
  align: (col, row) => (right,right,right,).at(col),
  inset: 6pt,
  [k], [p], [r],
  [18],
  [262,133],
  [32,887],
  [20],
  [1,048,573],
  [131,203],
  [22],
  [4,194,301],
  [524,731],
  [24],
  [16,777,213],
  [2,097,349],
)
]

The full per-cell table is in #link("data/tables.md")[`data/tables.md`],
with the census, T, the solving degree and the largest matrix width. The
primary series are below. T is the mean `field_ops` per system and L is
the number of chain levels. T is in F4 `field_ops` only (section 8 lists
the unpriced work). All values in the table are measured, from
`cells/*.json` and `data/census.json` \[pcc:unverified\].

#align(center)[#table(
  columns: 9,
  align: (col, row) => (right,right,right,right,right,right,right,right,left,).at(col),
  inset: 6pt,
  [k], [m], [d \= |V|], [L], [columns], [covered (m)], [T], [solving
  degree], [status],
  [20],
  [2],
  [3],
  [1],
  [2],
  [2],
  [1,648],
  [4],
  [complete],
  [20],
  [2],
  [7],
  [1],
  [2],
  [2],
  [11,225],
  [8],
  [complete],
  [20],
  [2],
  [9],
  [2],
  [4],
  [8],
  [52,244],
  [5],
  [complete],
  [20],
  [2],
  [19],
  [1],
  [12],
  [40],
  [96,745],
  [20],
  [complete],
  [20],
  [2],
  [21],
  [2],
  [8],
  [20],
  [434,893],
  [9],
  [complete],
  [20],
  [2],
  [27],
  [3],
  [16],
  [68],
  [1,789,928],
  [5],
  [complete],
  [20],
  [2],
  [57],
  [2],
  [34],
  [314],
  [3,876,409],
  [21],
  [complete],
  [20],
  [2],
  [63],
  [3],
  [28],
  [200],
  [22,317,287],
  [9],
  [complete],
  [20],
  [2],
  [73],
  [1],
  [36],
  [328],
  [1,869,761],
  [74],
  [complete],
  [20],
  [2],
  [133],
  [2],
  [76],
  [1,452],
  [35,142,880],
  [22],
  [complete],
  [20],
  [2],
  [171],
  [3],
  [108],
  [3,024],
  [383,236,931],
  [21],
  [complete],
  [20],
  [2],
  [189],
  [4],
  [96],
  [2,288],
  [393,654,078],
  [10],
  [complete],
  [20],
  [2],
  [219],
  [2],
  [120],
  [3,552],
  [61,803,451],
  [75],
  [complete],
  [20],
  [2],
  [399],
  [3],
  [216],
  [11,254],
  [2,016,013,118],
  [22],
  [complete],
  [20],
  [2],
  [511],
  [2],
  [274],
  [17,640],
  [1,463,328,780],
  [76],
  [complete],
  [20],
  [2],
  [513],
  [4],
  [278],
  [18,564],
  [4,027,107,095],
  [22],
  [complete],
  [20],
  [3],
  [3],
  [1],
  [2],
  [0],
  [19,010],
  [4],
  [complete],
  [20],
  [3],
  [7],
  [1],
  [2],
  [4],
  [3,303,793],
  [8],
  [complete],
  [20],
  [3],
  [9],
  [2],
  [4],
  [0],
  [147,635,021],
  [7],
  [complete],
  [20],
  [3],
  [19],
  [1],
  [12],
  [144],
  [220,436,708],
  [20],
  [complete],
  [20],
  [3],
  [21],
  [2],
  [8],
  [56],
  [2,787,948,088],
  [11],
  [complete],
  [20],
  [3],
  [27],
  [3],
  [16],
  [344],
  [263,459,825,507],
  [8],
  [complete (rerun after restart)],
  [20],
  [3],
  [57],
  [2],
  [34],
  [3,124],
  [-],
  [-],
  [censored (1200 s cap, first target)],
  [20],
  [4],
  [3],
  [1],
  [2],
  [4],
  [423,322],
  [8],
  [complete],
  [20],
  [4],
  [7],
  [1],
  [2],
  [4],
  [24,975,907],
  [8],
  [complete],
  [20],
  [4],
  [9],
  [2],
  [4],
  [24],
  [142,058,000,763],
  [9],
  [complete],
  [20],
  [4],
  [19],
  [1],
  [12],
  [504],
  [-],
  [-],
  [censored (out of memory)],
  [22],
  [3],
  [3],
  [1],
  [3],
  [0],
  [19,010],
  [4],
  [complete],
  [22],
  [3],
  [5],
  [1],
  [3],
  [4],
  [201,745],
  [6],
  [complete],
  [22],
  [3],
  [11],
  [1],
  [7],
  [32],
  [20,063,347],
  [12],
  [complete],
  [22],
  [3],
  [15],
  [2],
  [9],
  [52],
  [1,759,715,051],
  [9],
  [complete],
  [22],
  [3],
  [25],
  [2],
  [19],
  [576],
  [79,458,456,936],
  [10],
  [complete],
  [22],
  [3],
  [31],
  [1],
  [11],
  [84],
  [2,016,009,937],
  [32],
  [complete],
  [22],
  [3],
  [33],
  [2],
  [17],
  [420],
  [91,887,410,934],
  [15],
  [complete],
  [22],
  [3],
  [41],
  [1],
  [23],
  [1,020],
  [7,342,801,721],
  [42],
  [complete],
  [22],
  [3],
  [55],
  [2],
  [29],
  [2,076],
  [-],
  [-],
  [censored (out of memory)],
)
]

The censored k \= 20, m \= 3, d \= 57 target had spent 8.3e10 field
operations when it was stopped at degree 23 with an 777,369 x 798,826
matrix (`cells/k20_m3_sym_d57_g0.json`). That partial count is not a T
value and is not fitted \[pcc:unverified\]. The m \= 2 series at k \=
22, 24 and 18, the plain-form ablations and the coset cells are in
`data/tables.md`.

#strong[Figure.] The figure plots T against |V| on log-log axes, with k
\= 20 on the left and k \= 22 on the right (`exponent_fit.svg`, from
`scripts/figure.py`). Filled markers are prime d and open markers are
composite d. The dotted vertical lines mark censored cells, and two
reference slopes (1/2 and 1) are drawn.

#image("exponent_fit.svg")

== 3. Fitted exponents
<3-fitted-exponents>
The preregistered estimator is the OLS fit of ln T on ln |V| over the
complete cells with d \>\= 9, with a two-sided 95% t-interval. It
requires at least 4 cells spanning at least 4x in |V|. Its results are
in `data/analysis.json`, written by the frozen `scripts/analyse.py`. The
table\'s fits come from the cells through that script, except the d \>\=
9 fits in the last column for the gated series, which the validator
computed despite the span gate (`validator-report.json`, C11 and E1).
For m \= 4 that column holds the validator\'s fit over all three
complete cells.

#align(center)[#table(
  columns: 6,
  align: (col, row) => (left,right,right,left,left,left,).at(col),
  inset: 6pt,
  [series], [cells in the fit], [s], [95% interval], [sensitivity fit
  (all d)], [d \>\= 9 fit despite the gate],
  [k \= 20, m \= 2, sym],
  [14],
  [2.77],
  [\[2.31, 3.23\]],
  [2.80 \[2.47, 3.12\] (16 cells)],
  [(primary)],
  [k \= 22, m \= 2, sym],
  [9],
  [2.81],
  [\[1.81, 3.80\]],
  [2.72 \[2.23, 3.21\] (11)],
  [(primary)],
  [k \= 24, m \= 2, sym],
  [4],
  [2.57],
  [\[0.91, 4.24\]],
  [2.43 \[1.89, 2.97\] (5)],
  [(primary)],
  [k \= 20, m \= 2, plain],
  [6],
  [3.20],
  [\[1.67, 4.72\]],
  [3.31 \[2.65, 3.97\] (8)],
  [(primary)],
  [k \= 18, m \= 2, sym],
  [3 cells in all],
  [no fit],
  [-],
  [local slopes 2.19, 3.91],
  [-],
  [k \= 20, m \= 3, sym],
  [4 (d \= 9 to 27, span 3.0x)],
  [no fit (span \< 4x)],
  [-],
  [6.44 \[3.49, 9.39\] (6)],
  [5.56 \[-8.75, 19.88\] (4)],
  [k \= 22, m \= 3, sym],
  [6 (d \= 11 to 41, span 3.7x)],
  [no fit (span \< 4x)],
  [-],
  [5.76 \[3.77, 7.75\] (8)],
  [4.51 \[-1.27, 10.29\] (6)],
  [k \= 20, m \= 3, plain],
  [3 before censoring],
  [no fit],
  [-],
  [6.48 \[2.45, 10.52\] (5)],
  [-],
  [k \= 20, m \= 4, sym],
  [1 (d \= 9)],
  [no fit],
  [-],
  [local slopes 4.81 (3 to 7), 34.4 (7 to 9)],
  [all three cells: 9.86 \[-59.9, 79.6\]],
)
]

The fits in this table are derived from measured cells
\[pcc:unverified\]. At m \= 3 only the sensitivity fits, which include
the boundary cells d \< 9, have lower bounds above 0; the d \>\= 9 fits
do not exclude s \< 0 at 95% \[pcc:unverified\]. The local slopes of the
k \= 20, m \= 3 series are 6.09, 15.1, 0.54, 25.4 and 18.1, in order of
d (`data/analysis.json`) \[pcc:unverified\].

#strong[Exploratory, not preregistered: the chain length matters.] T
depends strongly on how d factors, not only on |V|. At equal |V|, a
composite d (more chain levels) costs more than a prime d. For example,
at k \= 20 and m \= 2, T is 1.87e6 at d \= 73 (L \= 1) and 2.23e7 at d
\= 63 (L \= 3) \[pcc:unverified\]. This explains some negative local
slopes in the preregistered series. Fits restricted to one chain length
are tighter. They come from `scripts/tables.py` and are stored in
`data/exploratory_fits.json`.

- m \= 2, L \= 1 gives 2.20 \[2.15, 2.25\] at k \= 20 (d \= 3, 7, 19,
  73), 2.19 \[2.14, 2.23\] at k \= 22 (5 cells) and 2.21 \[2.13, 2.28\]
  at k \= 24 (3 cells) \[pcc:unverified\].
- m \= 2, L \= 2 gives 2.43 \[2.10, 2.77\] at k \= 20 (6 cells)
  \[pcc:unverified\].
- m \= 3, L \= 1 gives 4.95 \[4.51, 5.40\] at k \= 22 (d \= 3, 5, 11,
  31, 41) and 5.04 \[-1.79, 11.88\] at k \= 20 (d \= 3, 7, 19)
  \[pcc:unverified\].

#strong[Exploratory, computed by the validator: regression on |V| and L
together.] The validator fitted ln T on ln d and L over all complete
cells of each series (`validator-report.json`, C10). The slopes in ln d
are 2.40 \[2.21, 2.59\] at k \= 20, m \= 2; 2.22 \[2.03, 2.41\] at k \=
22, m \= 2; 2.19 \[2.02, 2.36\] at k \= 24, m \= 2; 4.77 \[3.35, 6.20\]
at k \= 20, m \= 3; and 4.98 \[4.40, 5.55\] at k \= 22, m \= 3
\[pcc:unverified\]. The L coefficient excludes 0 in every series (1.01,
1.29, 1.18, 2.68 and 3.84) \[pcc:unverified\]. Adding L lowers every
exponent, but every slope stays far above the thresholds (s \< 0 at m \=
3, s \< 0.5 at m \= 4) \[pcc:unverified\]. The preregistered
single-regressor estimator stays primary; this fit is exploratory.

The solving degree is d + 1 for prime d in the sym form at m \= 2 and m
\= 3 \[pcc:unverified\]. It is not d + 1 in 7 prime-d cells: k \= 20, m
\= 4, sym, d \= 3 gives 8; the plain form gives 5, 9 and 21 at m \= 2 (d
\= 3, 7, 19) and 12, 15 and 27 at m \= 3 (d \= 3, 7, 19)
\[pcc:unverified\]. Where it holds, the cost is that of one dense
equation of degree about d in each chain variable \[pcc:unverified\].

#strong[Comparison with the tower base (L10).] L10 measured m \= 2 on
the isogeny factor base of the tower engine, with guessing G \= t - 1.
There T grows as about |V|^1.5: mpt 17,592, 52,816 and 146,464 at t \=
4, 5, 6, in tower-engine mul-adds. The G \= 0 comparison below did not
use L10\'s base: it ran the bit-interval base, on the same curve (k \=
20), in both engines. The tower engine\'s `f4cost` gives 87,013,
4,046,086 and 127,284,443 mul-adds per target at t \= 3, 4, 5
(`data/calib_tower_bits_t*.json`) \[pcc:unverified\]. These are local
slopes of 5.5 and 5.0 over |V| \= 2^t \= 8 to 32, in tower-engine
mul-adds (derived) \[pcc:unverified\]. This engine\'s `f4sub` gives
294,981, 10,458,148 and 178,989,665 field\_ops on the same base
(`data/calib_sub_bits_t*.json`), local slopes of 5.15 and 4.10 in
field\_ops over the same |V| range (derived) \[pcc:unverified\]. The
subgroup base\'s s \= 2.77 is a fit over |V| \= 9 to 513, so the slopes
cover different ranges. At G \= 0 and equal |V| the subgroup base is
cheaper than the bit-interval base, both in field\_ops: 1.79e8 at |V| \=
32 (bits) against 1.79e6 at d \= 27 \[pcc:unverified\]. L10\'s slope
with guessing on the isogeny base, about 1.5, is below this base\'s G \=
0 slope of 2.77 (2.20 at L \= 1), so L10\'s guessing strategy grows more
slowly than the subgroup base without guessing \[pcc:unverified\]. All
of these slopes are far above the 1/2 that a lever at m \= 4 needs
\[pcc:unverified\].

== 4. Balanced totals under the paper\'s model (derived)
<4-balanced-totals-under-the-papers-model-derived>
The relation phase costs p^(1 - (m-1)a) T(p^a) \= p^(1 - (m-1)a + s a),
and linear algebra costs p^(2a). The protocol\'s balance point is a \=
1/(m + 1 - s), with total exponent 2/(m + 1 - s). When s \>\= m - 1, the
relation-phase exponent 1 + (s - (m - 1)) a does not decrease as a
grows. No base size then gives a total below p^1, and the balance either
lands above 1 or has no positive solution.

#align(center)[#table(
  columns: 4,
  align: (col, row) => (right,left,left,left,).at(col),
  inset: 6pt,
  [m], [s used], [2/(m + 1 - s)], [best total exponent under the model],
  [2],
  [2.7732 (k \= 20 fit)],
  [2/0.2268 \= 8.82],
  [1 (no base size helps; s \>\= m - 1 \= 1)],
  [2],
  [2.8064 / 2.5745 (k \= 22 / 24)],
  [10.33 / 4.70],
  [1],
  [3],
  [6.44 / 5.76 (sensitivity fits)],
  [no positive solution (m + 1 - s \< 0)],
  [1 (s \>\= m - 1 \= 2)],
  [3],
  [4.95 (k \= 22, L \= 1)],
  [no positive solution],
  [1],
  [4],
  [4.812 (local slope, 3 to 7)],
  [2/0.188 \= 10.64],
  [1 (s \>\= m - 1 \= 3)],
  [4],
  [34.4 (local slope, 7 to 9)],
  [no positive solution],
  [1],
)
]

All values in this table are derived from fixed small p and are
extrapolations, not measurements \[pcc:unverified\]. The balanced-total
exponents use the unrounded slopes, because the value of 2/(m + 1 - s)
is sensitive to rounding near its pole. A total below p^(1/2) at m \= 4
would need s \< 0.5 at the paper\'s a \= 1/5, or s \< 1 at the
re-balanced a. A total below p^1 would need s \< 3 \[pcc:unverified\].

== 5. Verdicts on the preregistered thresholds and predictions
<5-verdicts-on-the-preregistered-thresholds-and-predictions>
#align(center)[#table(
  columns: 3,
  align: (col, row) => (left,left,left,).at(col),
  inset: 6pt,
  [item], [preregistered rule], [outcome],
  [lever at m \= 4],
  [s \< 0.5 with upper 95% bound \< 0.5],
  [#strong[undetermined] by the estimator: one complete cell with d \>\=
  9, and d \= 19 censored. Not met on any reading (local slopes 4.81,
  34.4)],
  [lever at m \= 3],
  [s \< 0 with upper bound \< 0],
  [#strong[undetermined] by the estimator: span 3.0x at k \= 20 and 3.7x
  at k \= 22, both below 4x. Not met on any reading: sensitivity fits
  6.44 \[3.49, 9.39\] and 5.76 \[3.77, 7.75\]; d \>\= 9 fits 5.56
  \[-8.75, 19.88\] and 4.51 \[-1.27, 10.29\], whose lower bounds do not
  exclude s \< 0],
  [looser derived m \= 4 break-even],
  [s \< 1],
  [not met on any reading],
  [prediction m \= 2: s \= 1.5 (range 1.0 to 2.3)],
  [],
  [#strong[failed]: 2.77 \[2.31, 3.23\] at k \= 20, outside the range],
  [prediction m \= 3: s \= 3.5 (range 2.5 to 5)],
  [],
  [not evaluable by the estimator. The readings 6.44 and 5.76 are above
  the range; the L \= 1 fits, 4.95 and 5.04, are inside it],
  [prediction m \= 4: s \>\= 4.5 if measurable (most likely capped
  beyond d \= 9 or 21)],
  [],
  [#strong[undetermined] for s (not measurable: no fit exists). Only the
  censoring expectation held: out of memory at d \= 19],
  [plain vs sym: |s\_plain - s\_sym| \< 0.5],
  [],
  [#strong[held]: 3.20 vs 2.77, a difference of 0.43],
  [coset dependence: T within 1.5x across gidx],
  [],
  [#strong[held]: at d \= 27, T is 1,789,928 / 1,789,919 / 1,789,928 for
  gidx \= 0, 1, 2; at d \= 63 it is 22,317,287 / 22,317,373 /
  22,317,372],
  [calibration: field\_ops / f4\_muladds within 5x],
  [],
  [#strong[held]: 3.39, 2.58 and 1.41 at t \= 3, 4, 5],
  [calibration: that ratio constant to 30%],
  [],
  [#strong[failed]: it falls from 3.39 to 1.41],
  [owner\'s expectation: no lever],
  [],
  [consistent: no reading comes near a threshold],
)
]

The outcomes in this table come from measured cells and the fits derived
from them \[pcc:unverified\]. Failed predictions stay failed.

#strong[Protocol eligibility ambiguity.] The protocol\'s clause \"with
at least 4 complete cells whose |V| spans at least 4x\" can also be read
as gating on all complete cells, not only on those with d \>\= 9. The
frozen `scripts/analyse.py` gates on the d \>\= 9 cells, and this study
follows it, so \"undetermined\" is the preregistered reading. Under the
other reading the d \>\= 9 fits (5.56 and 4.51 above) would be computed,
and the m \= 3 lever would read \"not met\" instead of \"undetermined\"
\[pcc:unverified\]. No lever is found under either reading
\[pcc:unverified\].

== 6. What a lever would need
<6-what-a-lever-would-need>
Under the paper\'s model, a lever at m \= 4 needs a decomposition system
whose cost grows more slowly than |V|^(1/2). At m \= 3 it needs one
whose cost does not grow at all. Measured here, the cost grows as |V|^2
or more steeply at m \= 2 on every series \[pcc:unverified\].

Trivial enumeration of V^(m-1), with a root test for the last summand,
costs |V|^(m-1). At m \= 4 that is already |V|^3, six times the exponent
a lever needs. A lever would therefore need a solver that neither
enumerates nor reduces over the |V|^m quotient of the chain ideal. This
study found no such solver \[pcc:unverified\].

The m \= 2 slope of the prime-d cells, about 2.2, is above even the
enumeration exponent 1. This suggests that the F4 route over the chain
ideal costs more than enumeration on this base (derived)
\[pcc:unverified\].

== 7. Prior exponent studies
<7-prior-exponent-studies>
#link("PRIOR_STUDIES.md")[`PRIOR_STUDIES.md`] lists five earlier
studies.

- `ic_exponent_20260926` fits the ratio to batch rho on binary Koblitz
  curves.
- `ic_m4_exponent_audit_20260928` measures c \= 0.985 bits per unit n
  for the m \= 4 decomposition over an F\_2-subspace base of binary
  fields.
- `ic_exponent_top_20260930` was withdrawn before anything ran.
- `ic_exponent_falsifiers_20261013` studies the exponent sum of base
  abscissae, not a cost.
- `ecdlp_exponent_assessment_20261018` has PDFs that could not be
  retrieved on this machine.

None of the readable four measures T(E, m, L) against the size of a
prime-field multiplicative-subgroup base, so none gives a number to
compare directly with s here. The closest is the m \= 4 audit, on a
binary field with an F\_2-subspace base. It measured c \= 0.985 bits per
unit n \[0.983, 1.018\] for the m \= 4 decomposition and recorded that
route as \"closed for this engine at these sizes\" (as summarised in
`PRIOR_STUDIES.md`). Its verdict points the same way as this study\'s
\[pcc:unverified\]. The m \= 2 tower-base values of L10 are compared in
section 3.

== 8. Distance to rho
<8-distance-to-rho>
This study does not produce an end-to-end cost. As an illustration only,
the modelled cost is E \= columns · r · T / covered, in F4 `field_ops`
(F\_p multiplications). The rho reference 1.5 sqrt(r) counts group
operations, not field\_ops. No conversion factor between the two units
was recorded or measured, so each ratio below is \"field\_ops per rho
group operation\", not a ratio of equal units. At k \= 20 and m \= 2,
the ratio of E to 1.5 sqrt(r) ranges from 3.98e5 field\_ops per rho
group operation (d \= 3) to 1.46e10 (d \= 513) (derived)
\[pcc:unverified\]. Native operations are unpriced, so the w band of
AGENTS.md section 5a is not applied. The ratios are not comparable with
L10\'s mul-add figures beyond the calibration ratio of 1.4 to 3.4. Rho
remains ahead \[pcc:unverified\].

#strong[Unpriced work.] T is the F4 row reduction only, as the protocol
defines it. The following work is not counted in T or in E:

- #strong[Per-target construction of the symmetrised system.]
  `build_system` calls `symmetrise` for every target, interpolating S~
  at that target X by Gauss-Jordan elimination on an (n + 12) x (n + 1)
  matrix, with n \= 6, 35 and 495 monomials at m \= 2, 3 and 4. The
  validator\'s count is about 4.6e2, 3.1e4 and 6.2e7 F\_p
  multiplications per target \[pcc:unverified\]. At m \= 4 this exceeds
  T at d \= 3 (4.2e5) and d \= 7 (2.5e7); at m \= 3 it exceeds T at d \=
  3 (1.9e4) \[pcc:unverified\]. It is constant in d and could be
  amortised to once per curve by interpolating in (e, X).
- #strong[Rho\'s native work and the conversion of group operations to
  field\_ops] (above).
- #strong[Census and system specialisation] outside F4 (enumeration of V
  and the covered set), which is an analysis aid, not part of T.

== 9. Not done and open issues
<9-not-done-and-open-issues>
- #strong[m \= 3 and m \= 4 exponents.] Neither could be fitted under
  the frozen rule. A wider span at m \= 3 needs more than the frozen 3.5
  GB and 1200 s per target. At k \= 22, d \= 55 exited out of memory. At
  k \= 20, d \= 57 hit the 1200 s wall-clock cap at degree 23, with a
  matrix of about 8e5 columns, on a host shared with another agent\'s
  build (see \"Host, load and wall time\"). A sparser linear-algebra
  back end would also help. This is a resource limit, not a result.
- #strong[Process items.] No `bench.record/v1` record exists, and the
  grid ran locally rather than on the EKS route. These are for the
  coordinator.
- #strong[m \= 4 stays open.] It has three complete cells (d \= 3, 7, 9)
  and is censored at d \= 19.
- #strong[Chain length.] The protocol did not anticipate that prime d
  and composite d behave differently. A follow-up should fix L, or
  regress on (ln |V|, L), in its preregistration, and should name the
  lower envelope over factorisations (prime d is cheapest at equal |V|)
  as the quantity relevant to a lever.
- #strong[Inconsistent systems.] Most targets give an inconsistent
  system, so T measures refutation cost. Consistent and inconsistent
  systems were not split, because the protocol does not ask for it.
- #strong[Unretrieved prior study.] The
  `ecdlp_exponent_assessment_20261018` PDFs live in the artifact store
  and were not retrievable on this machine.
- #strong[Unavailable tools.] research-kb, ecdb and Conductor were
  unavailable in this session: there was no MCP server and no token, and
  Conductor is not installed. The sources checked were the repository
  files named in `PROTOCOL.md` and `PRIOR_STUDIES.md`.
- #strong[No pcc claims.] None were registered, so every result is an
  unverified assertion until checked.

== Files
<files>
#align(center)[#table(
  columns: 2,
  align: (col, row) => (left,left,).at(col),
  inset: 6pt,
  [file], [what],
  [`PROTOCOL.md`, `data/frozen.sha256`],
  [the frozen protocol and the hashes of the protocol, the scripts and
  the binary],
  [`PRIOR_STUDIES.md`],
  [the addendum on the five earlier exponent studies],
  [`validator-report.json`],
  [the independent validation (errata E1 to E13), committed unchanged],
  [`scripts/run_grid.py`],
  [the grid runner (resumable per cell)],
  [`scripts/analyse.py`],
  [the preregistered estimator; writes `data/analysis.json`],
  [`scripts/census.py`],
  [the exact census; writes `data/census.json`],
  [`scripts/tables.py`],
  [`data/tables.md` and the exploratory fits at fixed L
  (`data/exploratory_fits.json`)],
  [`scripts/figure.py`],
  [`exponent_fit.svg` and `.png`],
  [`cells/*.json`],
  [one file per cell, with per-target field\_ops, solving degree, rows
  and columns],
  [`data/calib_*.json`],
  [the calibration cells (bit-interval base, both engines)],
  [`data/run.log`],
  [the runner\'s log, with the resume note of 2026-11-10],
  [`REPORT.typ`, `REPORT.pdf`],
  [the report (pandoc to Typst, compiled with the typst Python module)],
)
]

Reproduce: use the archived frozen binary (sha256 `dc2c30a4…3faf`, see
\"Source, toolchain and binary\"), or check out `8d0388b10a` and run
`cargo build --release --bin curve25519_family` with `rustc 1.97.0`; a
rebuild is not guaranteed to reproduce the frozen hash. Then run from
this directory, in order:

+ `python3 scripts/run_grid.py`
+ `python3 scripts/census.py`
+ `python3 scripts/analyse.py`
+ `python3 scripts/tables.py`
+ `python3 scripts/figure.py`

== Validation
<validation>
An independent validator (a separate sub-agent that did not produce the
study) checked commit `8c544a1829` and wrote
#link("validator-report.json")[`validator-report.json`], committed
unchanged. Its status is \"pass with errata\": every measured number and
preregistered verdict it checked recomputes, 56 of 70 cells replay
identically with the frozen binary, and none of its 13 errata changes a
preregistered verdict or a fitted number (as reported by the validator)
\[pcc:unverified\]. The validation is an unsigned check, not a pcc
claim. Each erratum was applied as follows.

#align(center)[#table(
  columns: 3,
  align: (col, row) => (left,left,left,).at(col),
  inset: 6pt,
  [id], [issue], [how it was applied],
  [E1],
  [\"far above both thresholds\" implied that m \= 3 data exclude a
  lever],
  [Answer, section 3 table (new column) and section 5 now give the d
  \>\= 9 fits 5.56 \[-8.75, 19.88\] and 4.51 \[-1.27, 10.29\] and the m
  \= 4 fit 9.86 \[-59.9, 79.6\], and say that only the sensitivity fits
  have lower bounds above 0],
  [E2],
  [headline lacked the resource-limit caveat],
  [Answer: \"no lever at m \= 2\"; m \= 3 and m \= 4 undetermined
  because of 3.5 GB virtual memory, 1,200 s wall per target, shared
  host],
  [E3],
  [unit-test description],
  [section 1: p \= 211, the \"consistent iff S\_{m+1} has a root in
  V^m\" property, `roots_of_unity_and_factors` listed, no m \= 4
  consistency test, log not committed],
  [E4],
  [L10 comparison mixed bases, ranges and units],
  [section 3: L10 used the isogeny base; the G \= 0 comparison ran the
  bit-interval base; slopes 5.5, 5.0 (tower mul-adds) and 5.15, 4.10
  (field\_ops) over |V| \= 8 to 32; L10\'s guessing slope of about 1.5
  is below this base\'s 2.77],
  [E5],
  [field\_ops compared with rho group operations],
  [section 8: ratios relabelled \"field\_ops per rho group operation\";
  no conversion factor recorded],
  [E6],
  [\"solving degree d + 1 for prime d\"],
  [section 3: restricted to sym, m \= 2 and 3; the 7 exceptions listed],
  [E7],
  [per-target symmetrisation unpriced],
  [section 8 \"Unpriced work\": about 4.6e2, 3.1e4 and 6.2e7
  multiplications at m \= 2, 3, 4, exceeding T at m \= 4 for d \= 3 and
  7 and at m \= 3 for d \= 3],
  [E8],
  [balanced totals from rounded s],
  [section 4: 8.82, 10.33, 4.70 and 10.64 from the unrounded slopes],
  [E9],
  [\"prediction m \= 4 held\"],
  [section 5: undetermined for s; only the censoring expectation held],
  [E10],
  [post-data `tables.py` change undisclosed; k \= 22 boundary cells],
  [\"Post-data change to a non-frozen script\" paragraph; Answer names d
  \= 3 and 5 for k \= 22],
  [E11],
  [no host or load record; wall-clock censoring; time gaps],
  [\"Host, load and wall time\" paragraph; wall time stated as not a
  claimed quantity],
  [E12],
  [binary only in a shared build directory; source and toolchain
  unpinned],
  [\"Source, toolchain and binary\": commit `8d0388b10a`,
  `rustc 1.97.0 (2d8144b78 2026-07-07)` (read on the host after the
  run), binary copied to
  `/home/user/bin-archive/curve25519_family-dc2c30a4` with the frozen
  sha256; not committed; artifact-store upload still open],
  [E13],
  [untagged result sentences],
  [sections 2 and 3: census, consistency, solving-degree and
  table-caption sentences tagged],
)
]

The validator also reported the chain-length regression (section 3,
exploratory) and the protocol-eligibility ambiguity (section 5); both
are now in the text. Two items stay open: the binary is not yet in the
artifact store, and the unit-test output is not committed (the tests
were not rerun, because building was not allowed).
