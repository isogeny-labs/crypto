# Degree-3 cover push-forward: every relation on the transferred subgroup is a full fibre

Protocol (frozen before data): [PROTOCOL.md](PROTOCOL.md), hash in `frozen.sha256`
(`ab23f1c56a559725e3c2f8b3d0418a8b705574b7c66accb3de2fdf19f3d56b25`). No amendment was needed.
Code: `src/cryptanalysis/chained_transfer/pushforward_deg3.rs`, `src/bin/cover_pushforward_deg3.rs`.
Data: `deg3.json` (all 24 cells, 360 seed-cells), `run.log`, `deg3.svg`.
Bench record: `benchmarks/records/20261112T2149Z-cover-pushforward-deg3.json` (left open; its recorded
command exited 1, see "What went wrong").
Every number below is a counted group operation or a count from those files and carries [pcc:unverified].
Counts are deterministic and exact; no timing claim is made.

## Verdict against the frozen rule

**The generalization holds at degree 3.** [pcc:unverified] At all 8 primes (53 to 251), in all 24 cells and
all 360 seed-cells: (1) 28,815 of 28,815 collected J(C) relations are a single full fibre `c^*(Q)` of the
cover (0 unions of fibres, 0 others); (2) the index calculus on J(C) and the native collision search on `E`
accept identical trials, cost identical counted group operations, collect identical numbers of rows and
recover the identical log in 360 of 360 seed-cells; (3) 0 of 28,815 pushed relations are invalid on `E`, and
every pushed row equals `[3]Q` (0 exceptions). [pcc:unverified]

So the index calculus on the transferred subgroup of the genus-4 Jacobian is a collision search on `E`
that keeps the trials whose fibre `h(x) = s` splits completely (about 1 in 6) and does three times the
bookkeeping per accepted row. It carries no information the points of `<G_E>` do not. [pcc:unverified]

The strongest form of the prediction also held trial by trial: in 159,508 of 159,508 trials the reduced class
of `[alpha]D + [beta]T` computed by Cantor arithmetic on J(C) equals the Mumford pair `(h(x) - s, Y)` of
`Q = [alpha]G_E + [beta]T_E` computed on `E` alone. [pcc:unverified]

## Mechanism as measured

Cover: `E : Y^2 = c(s)`, `h(x) = x^3 + h1 x + h0`, `C : y^2 = c(h(x))` of genus 4, `c(x, y) = (h(x), y)` of
degree 3 (the `tower_ladder` rung 4; `HCurve`, so `p < 256`). The pull-back homomorphism was checked for every
`k` in `1..r` in each of the 24 instances (854 checks in total) before any relation was collected.
[pcc:unverified]

1. **The reduced class is a fibre.** `c^*(Q)` has degree 3, at most `g = 4`, so its reduced Mumford
   representative is unique, and `c^*` is a homomorphism: `[alpha]D + [beta]T = c^*(Q)`. Measured: the class
   equals `(h(x) - s, Y)` in 159,508 of 159,508 trials, identity classes included (4,706 trials gave `Q = O`).
2. **It splits exactly when the fibre does.** The class has all its roots in `F_p` exactly when `h(x) = s` does,
   and then its places are the whole fibre. The relations are rows with three entries of equal sign (sign
   from `Y`), one per place over `Q`. Measured acceptance rate over non-identity trials, pooled:
   0.1861 (28,815 rows in 154,802 trials), against `|B3| / #E` pooled 0.1702 and 1/6 = 0.1667.
   The pooled rate is higher than 1/6 because the subgroup `<G_E>` is small (`r` from 11 to 83) and each cell
   was admitted only if at least two of its subgroup abscissae lie in `B3`. [pcc:unverified]
3. **Classification.** Every accepted row is a single full fibre. This includes 1,317 rows whose fibre has a
   repeated root (a ramified fibre, `2P + P'`), which occurred in 3 of the 24 cells (18 ramified fibres
   among all the points of `E`, all of them completely rational). [pcc:unverified]
4. **Push-forward.** `c_*` of each row, by independent point arithmetic on `E` (`sum m_i (h(x_i), y_i)`), equals
   `[3 alpha]G_E + [3 beta]T_E` and equals `[3]Q` in 28,815 of 28,815 rows. The pushed relation is the
   degenerate `Q + Q + Q`, not a three-term decomposition over independent elements. [pcc:unverified]
5. **Fibre statistics.** Over all 24 curves `E`, the number of distinct rational places above an affine
   point is 0, 1 or 3, with 2 only where the fibre has a double root: `|B1|` from the places of `C` equals
   `|B1|` from `E` and `h` alone in 24 of 24 cells. The images of rational places have fibre size 1 in
   1,698 cases, 2 in 18 cases (the ramified ones) and 3 in 565 cases. `|B3| / #E` lies between 0.0909 and
   0.2456 per cell. `|B3 cap <G_E>| / (r - 1)` pooled is 0.1991 (columns up to sign: 2 to 9 per cell).
   [pcc:unverified]
6. **The matrix is a collision table.** The J(C) arm has 26 to 133 columns (one per degree-1 place with
   `y <= (p-1)/2`) and the E arm 2 to 9 (one per `+-Q` in `B3 cap <G_E>`); the elimination returns the log through a
   repeated `Q` in both. Median relation count at the first repeated `Q`: 3. [pcc:unverified]

## Results

Cells: for each prime, the first three admissible instances `tower::build(p, 11 + i + 37 j)` (20 candidates
were skipped, 17 for fewer than 2 subgroup columns in `B3`, 3 because `tower::build` returned nothing; the
`tower_ladder` instance `j = 0` is in 5 of 8 primes). `r` is 11 to 83 (13 distinct values). 15 stream seeds per
cell. The one-time construction (tower, cover search, homomorphism checks) took 1.9 to 80.6 s per instance and
is in no total. [pcc:unverified]

### Per prime (3 instances x 15 seeds = 45 seed-cells each; group operations; all cells [pcc:unverified])

J(C) and E fibre arms have identical counted operations in every seed-cell, so one column serves both. Gap
columns are mean total over mean matched rho (`rho_reference`) with the linear algebra priced at w = 0 / 0.1 / 1
GAE per `Z/r` multiplication: the distance remaining, not a pass/fail gate.

| p | cells (r) | B3/#E | accept rate | rows/seed | trials/seed | J(C)=E ops (mean) | rho (mean) | gap w=0 / 0.1 / 1 | first collision J(C) median | birthday median | rho median | seeds J(C) first collision below rho |
|--:|--|--:|--:|--:|--:|--:|--:|--|--:|--:|--:|--:|
| 53 | 3 (11, 31, 31) | 0.1667 | 0.2707 | 37.3 | 144.6 | 1315.0 | 198.8 | 6.61 / 7.21 / 12.53 | 71 | 41 | 214 | 42/45 |
| 79 | 3 (41, 11, 19) | 0.1429 | 0.1692 | 44.3 | 271.5 | 2649.5 | 196.1 | 13.51 / 14.41 / 22.47 | 102 | 32 | 177 | 38/45 |
| 101 | 3 (19, 23, 17) | 0.2078 | 0.2490 | 64.3 | 273.1 | 2193.0 | 195.6 | 11.21 / 13.16 / 30.70 | 80 | 37 | 189 | 42/45 |
| 131 | 3 (67, 31, 59) | 0.1702 | 0.1649 | 70.7 | 437.9 | 5134.4 | 303.3 | 16.93 / 18.45 / 32.14 | 171 | 58 | 285 | 32/45 |
| 151 | 3 (71, 43, 17) | 0.1689 | 0.1954 | 81.0 | 426.5 | 4720.6 | 260.6 | 18.12 / 20.51 / 42.00 | 183 | 62 | 269 | 32/45 |
| 181 | 3 (41, 43, 83) | 0.1633 | 0.1690 | 90.3 | 546.0 | 6535.6 | 310.1 | 21.08 / 23.54 / 45.69 | 202 | 81 | 293 | 35/45 |
| 211 | 3 (31, 59, 29) | 0.1514 | 0.1348 | 113.7 | 864.5 | 9648.8 | 264.7 | 36.46 / 41.19 / 83.82 | 180 | 67 | 241 | 31/45 |
| 251 | 3 (37, 41, 23) | 0.1855 | 0.2474 | 138.7 | 580.6 | 5717.2 | 244.4 | 23.39 / 30.92 / 98.69 | 100 | 46 | 233 | 39/45 |

Per cell (`run.log`), the full-stop gap to the mean matched rho of the same cell ranges from 2.49 to 48.27
at w = 0, from 3.18 to 53.45 at w = 0.1 and from 9.38 to 121.37 at w = 1 for J(C)
(the E arm has the same w = 0 gap and fewer linear-algebra multiplications, a median ratio J(C) to E of 13.5, so its w = 0.1 and w = 1
gaps are smaller: the arms tie at w = 0 and E is cheaper at w = 0.1 and 1). The full-stop
J(C) arm was below rho on 0 of 360 seeds. [pcc:unverified]

### First collision (360 seed-cells; group operations; all [pcc:unverified])

Reading the J(C) relation list at the first repeated `+-Q` gives exactly the E arm's first collision in 360 of
360 seed-cells. Pooled medians: J(C) and E arm 134, birthday control (no cover, no filter, same stream) 53,
`rho_reference` 233. The J(C) first collision is below the rho of the same seed on 291 of 360 seeds, the
birthday control on 359 of 360. The control is at or below the cover arms on every seed. The median over
cells of first-collision cost is below the cell's rho median in 22 of 24 cells (exceptions: p = 131 instance
`j = 3` and p = 211 instance `j = 3`, both r = 59). The first-collision read costs a median of 3.5% of the full-stop
group operations. This is the same picture as for degree 2: the cheap result is a birthday search on `E` which
the cover only slows, by discarding about 5 of 6 trials. [pcc:unverified]

### Against the A = 2 reference (negation map), for the distance remaining

`rho_reference_negation` is also run on the same seeds. It recovered the log in 354 of 360 seed-cells; in the
other 6 (p = 53 `j = 4`: 2, `j = 6`: 1; p = 79 `j = 4`: 1, `j = 5`: 2) it reached the 2^24 step cap, and those seeds are
left out of its statistics. Median over the 354: 50 group operations (range 25 to 673), against 233 for
`rho_reference`. Against it the full-stop J(C) gap is 11.2 to 268.7 per cell at w = 0 (median 95.97)
and the J(C) first collision is below it on 41 of 354 seeds, the birthday control on 156 of 354.
[pcc:unverified]

## Prediction against outcome (scorecard)

| id | frozen prediction (probability) | outcome |
|:--|:--|:--|
| P1 | every accepted row is a single full fibre (0.97) | held: 28,815 of 28,815 |
| P2 | every trial's class equals `c^*(Q)` (0.98) | held: 159,508 of 159,508 |
| P3 | A1 and A4 identical trial sets, identical counted ops, rows, log (0.97 / 0.99 / 0.97) | held in 360 of 360 seed-cells |
| P4 | all pushed rows valid on `E`, equal `[3]Q` (0.98) | held: 0 invalid, 0 not `[3]Q` |
| P5 | pooled acceptance in [0.12, 0.21], pooled `B3/#E` in [0.14, 0.20], each cell's in [0.08, 0.27] (0.85) | held: 0.1861; 0.1702; 0.0909 to 0.2456 |
| P6 | fibre sizes 0, 1, 3 and 2 only at ramified points (0.97) | held: 24 of 24 cells, 18 size-2 images, all ramified |
| P7 | every recovered log in A1, A4, A5 certified (0.97) | held: 0 failures in 360 seed-cells for each |
| P8 | full-stop arms exceed rho everywhere and at all three prices (0.90) | held: smallest cell gap 2.49 (w = 0) |
| P9 | first collision below rho median in at least half the cells (0.55); A5 at or below A4 per seed (0.90) | held: 22 of 24 cells; 360 of 360 seeds |

All values [pcc:unverified]. Success criterion (owner, 2026-10-15): no measured improvement over the
index-calculus baseline here. The cover arms are a relabelled collision search on `E`; the first collision of J(C)
or `E` beats `rho_reference` in a majority of seeds at these tiny sizes, but the birthday control with no cover
does so more often and cheaper, so the cover contributes nothing. Distance remaining to matched rho: full stop
6.61x to 36.46x (w = 0, per-prime mean ratio), the first-collision read about 0.6x
(pooled median 134 against 233) and not below the no-cover control. [pcc:unverified]

## Why a degree-d cover cannot help here, in this construction

`<G_E>` pulls back into `c^*(E)`. A divisor class of degree `d <= g` has a unique reduced representative, and in
`c^*(E)` it is the full fibre of one point. A relation with fewer than `d` places, or with places from different fibres,
cannot occur: no row of the 28,815 was of either kind. So the weight of every relation is exactly `d`
and the factor base is the set of fibres, which is a subset of `E`, of density 1/6 for a generic `h`
(`|B3 cap <G_E>| / (r - 1)` pooled 0.1991). A shortcut would need relations that leave `c^*(E)`, as the
factor-base walk of `hyper_ic` does (it adds factor-base places to the walk, so its divisors are not in the
subgroup); those relations are not covered by this experiment. [pcc:unverified]

## Limits and what went wrong

- `HCurve` stores `u8` coefficients and genus at most 4, so `p < 256` and one cover shape. Larger `p` at genus 4 needs a
  new Jacobian implementation, not written here. `r` is the largest prime exactly dividing `#E` from
  `tower::build`: 11 to 83, so the subgroup is tiny and the absolute counts are small.
- Selection: a candidate instance was admitted only if at least two abscissae of the subgroup lie in `B3`
  (frozen in the protocol). This raises the pooled acceptance rate above 1/6 (0.1861) and is why cells with
  `r = 11` accept 42% and 61% of non-identity trials (the two `r = 11` cells).
- Only degree 3, weight-3 relations on the transferred subgroup, one generic `h` per instance (no
  special monodromy). Nothing here speaks for covers with non-generic fibre structure or for relations outside
  `c^*(E)`.
- The brute-force root search used to split divisors and fibres is unpriced native work (not in any total).
  The library ledger (`adds + doubles`) on J(C) exceeds the counted convention by about two operations per trial (the
  first addition into an identity accumulator that the convention does not charge); the arms' identical counts
  are under the shared convention. [pcc:unverified]
- The independence of the push check: `c_*` on `E` is a sum of images `(h(x), y)` of the places; the comparison
  sides are computed on `E` alone. The splitting of the J(C) class and of the E fibre both use the same
  brute-force root routine (on different polynomials: `u` from Cantor arithmetic, `h - s` from `E`).
- What went wrong: the recorded command exited with status 1. The binary's exit status folds in the
  `rho_reference_negation` recoveries, which failed in 6 of 360 seed-cells (step cap); all arms under test
  (J(C), E fibre, birthday control, `rho_reference`) were certified in 360 of 360. The raw files were not
  edited; the status is recorded as is. A smoke run of the binary on p = 53 (one instance, one seed, no record, about 11 s) preceded the
  full run only to check that it ran; no number from it is used and no protocol change followed.
- Not done here (belongs to the integration step): regeneration of `benchmarks/progress/` and the benchmark
  README row; the bench record is left open. `cargo clippy --release --all-targets -- -D warnings` for the whole
  crate is reported in the hand-off.
