# Small D_E with an endomorphism-adapted factor base: measured relation cost (2026-11-12)

Protocol: [`PROTOCOL.md`](PROTOCOL.md), frozen by hash before any measured run
([`frozen.sha256`](frozen.sha256), re-verified after the run: `PROTOCOL.md: OK`)
with Amendment 1 (pre-data) pinning cells, bases, solvers, costs and the stop
rule. Code: `src/cryptanalysis/chained_transfer/small_de.rs`,
`src/bin/small_de_ic.rs`; renderer `render.py`. Bench record:
`benchmarks/records/20261112T1753Z-small-de-endo-fb.json` (open). Raw data:
`small_de.json`, `run.log` (recorded); derived: `small_de_derived.json`,
`tables.md`, `small_de.svg` (total relation-phase cost per arm vs p, one panel
per D). Every number below is an unverified assertion produced by that run
`[pcc:unverified]`; none is signed by pcc.

## Answer

Question: after choosing coordinates and a factor base adapted to a low-degree
endomorphism, does small `D_E` reduce the actual solving cost? No. The cost of
solving one trial (the `S_3` quadratic for `m = 2`, the `S_4` quartic as two
nested quadratics for `m = 3`) is the same in every arm of every cell, to the
operation for `m = 2` and within 0.85% for `m = 3` (correction 2026-11-13, from
the independent validation: 12 of 60 `m = 3` cells vary between repetitions,
and A2/A1 is 1.0007 in one cell): field operations per trial are identical for A2 and A1 (ratio
1.000) because the sweep runs over the same number of base points
`[pcc:unverified]`. The decomposition probability per trial is unchanged for
`D = -3, -4` (19 of 20 cells within 2 standard errors, the other at 2.7)
and is lower by 0.09 to 0.11 at `m = 2` and 20 to 28 bits for the layered
`D = -7, -8` bases `[pcc:unverified]`. What does fall is the number of columns, hence the
relations needed and the elimination: A2 has exactly 1/3 (`D = -3`), 1/2
(`D = -4`) or 1/4 (layered `D = -7, -8, -11`) of A1's columns, and its
relation-phase cost at w = 0.1 is 0.33 to 0.35, 0.47 to 0.56 and 0.24 to 0.39
of A1's at 20 to 28 bits `[pcc:unverified]`.

That gain is the folding, not the small `D_E`. The control A3, a generic curve
(`|d_K|` above 100) whose base is folded by multiplication with a random scalar
in the same four layers, has relation-phase cost 0.22 to 0.30 of its x-interval
twin A0 at 20 to 28 bits, and the layered `D = -7, -8, -11` arms differ from it
by at most 0.097 at 24 and 28 bits (22 of 30 cells within 0.1 over all sizes)
`[pcc:unverified]`. The automorphism arms (`D = -3, -4`) are beaten by the
generic scalar-folded control (0.33 and 0.5 against 0.22 to 0.30), because the
fold factor the control uses (4) is a free parameter while `|Aut|/2` is 3 and 2
`[pcc:unverified]`.

Against the matched rho the gap stays large and grows with `p`: no IC arm is
below its matched rho at any weight in any of the 120 arm-cells; the smallest
single-instance ratio is 2.32 (w = 0), 7.62 (w = 0.1) and 40.16 (w = 1); at
28 bits the amortised ratio (w = 0.1) is 125 to 1631 across arms
`[pcc:unverified]`. Between 16 and 28 bits the `m = 2` amortised ratio grows by
a factor of 45 (A0), 61 (A2, `D = -3`) and 106 (A1, `D = -3`), against
`sqrt(2^12) = 64`, the `p` versus `sqrt(p)` gap `[pcc:unverified]`. Distance
remaining, not a gate: the best row is A2 for `D = -11`, `m = 2`, 12 bits, at
2.29 times the matched rho amortised over 16 targets (w = 0.1)
`[pcc:unverified]`.

## Verdict against the frozen decision rule

Rule (Amendment 1): A2 is a gain for a `D` if in every `(bits, m)` cell its
relation-phase total is below A1's at all three weights by more than two pooled
standard errors, with every target certified and the 90% rule met.

- `D = -3, -4, -7, -8`: gain. All 10 cells of each are `win / win / win`,
  16 of 16 targets certified in every arm of every cell (60 cells), the 90%
  rule met in every repetition `[pcc:unverified]`.
- `D = -11`: not a gain by the rule as written. The five `m = 2` cells are
  `loss / win / win`: at w = 0 (native work free) A2 costs more than A1 because
  building the layered base needs two priced group additions per image (degree-3
  Velu kernel sum) and the cell's priced trial cost is small; at w = 0.1 and 1
  A2 wins. The order changes inside the band, so the pair is unresolved at that
  boundary and not a win `[pcc:unverified]`.
- The rule passes for four `D`, but the control shows the pass is the fold
  factor, not `D_E` (previous section). The rule does not ask for that
  comparison; the control arm was added after the smoke test and is labelled so
  in Amendment 1.

## Prediction scorecard (predictions frozen before data, in PROTOCOL.md)

1. `D = -3, -4`, P = 0.7: **partly refuted.** Columns fall by exactly 3 and 2
   (holds). Decomposition probability and solving work unchanged (holds: ratio
   1.000 of ops per trial; probability within 2 SE in 19 of 20 cells). The
   elimination falls by "about its square" (**refuted**): the ratio A1/A2 of
   elimination multiply-adds is 2.7 to 3.3 (`D = -3`, `m = 2`) and 2.0 to 2.1
   (`D = -4`, `m = 2`), i.e. about the fold factor, because the rows stay
   2 to 3 entries wide and the sparse elimination is nearly linear; at `m = 3`
   it is 3.1 to 7.1 and 2.0 to 3.3 against 9 and 4 predicted. The gap to the
   automorphism-matched rho "at least the plain-IC gap" (**refuted**): A2 over
   the `A = 6, 4` rho is at least A1 over plain rho in 10 of 20 cells at w = 0
   and in none at w = 0.1 or 1 (A2 over the automorphism rho: minimum 2.73,
   12.4, 42.3 for w = 0, 0.1, 1; A1 over plain rho: minimum 2.33, 25.3, 167)
   `[pcc:unverified]`.
2. `D = -7, -8, -11`, P = 0.65: **refuted.** A2 costs less than A1 in the
   relation phase at w = 0.1 and 1 in all 30 cells and at w = 0 in 25 of 30
   (the five exceptions are the `D = -11`, `m = 2` cells) `[pcc:unverified]`.
   The reasoning behind the prediction (no useful base is closed under `psi`
   without growing by about `deg psi` per step) is right as a statement about
   closure: `lambda^k = 1` for some `k <= 1000` only at 12 bits for `D = -7, -11`
   (orders 556 and 27) and not at all above, so no finite closed base of useful
   size exists; but a layered base, closed only up to its top layer, folds
   columns by its layer count and that is all the effect needs
   `[pcc:unverified]`.
3. No cell ranks A2 below the matched rho, P = 0.85: **holds**, 0 of 120
   arm-cells below the best matched rho at any weight `[pcc:unverified]`.
4. Control (added after the smoke test, P = 0.8): `A3/A0` equals `A2/A1` for
   `D = -7, -8, -11` within noise: **holds at 24 and 28 bits** (max difference
   0.097 over 12 cells), 22 of 30 cells within 0.1 over all sizes; the
   eight others are all at 12 and 16 bits (fixed base-build costs weigh more;
   the largest is `D = -11`, `m = 2`, 12 bits: 0.325 against 0.685)
   `[pcc:unverified]`.

## Other measured facts

- The solving cost per trial depends only on the number of base points and `m`:
  for example 5209 field operations at 20 bits and 90546 at 28 bits for
  `D = -11`, `m = 2`; 50148 and 2137896 for `m = 3` (the nested quadratics),
  with the same value in A0, A1, A2 and A3 of a cell (for `m = 3` within 0.85%; see the correction above) `[pcc:unverified]`.
  The degree of the univariate solved is 2 for `m = 2` and 4 (as 2 x 2) for
  `m = 3`; no Groebner or Macaulay matrix is built, so there is no largest
  matrix for the solving step, only the elimination sizes in the tables.
- The layered `D = -7, -8` bases have a lower share of trials with at least one
  decomposition at `m = 2` (about 0.77 and 0.76 against 0.87 at 28 bits) with the
  same hits per trial (2.04 and 1.95 against 2.00 and 2.02), so the hits come in
  clusters `[pcc:unverified]`. Cause not tested. A guess, not a result: the
  characteristic polynomial of `psi` makes the layers integer combinations of
  two of them (`psi^2 = psi - 2` for `D = -7`, `psi^2 = -2` for `D = -8`).
  `D = -11` (`psi^2 = psi - 3`) shows no such drop (0.869 against 0.862)
  `[pcc:unverified]`.
- Stop rule: the 90% rule was met in all 1416 repetitions; 562 stopped at the
  first checkpoint (1.0 times the columns), 786 at the second (1.5), and 68
  needed 3 to 6 attempts (at most 6). Relations needed are therefore about 1.0
  to 1.5 times the column count `[pcc:unverified]`.
- Matched rho recovered the log in all 16 targets for each of the 140 rho
  variant-cells, each certified `[k]G == T` `[pcc:unverified]`.

# Tables (generated by render.py)

<!-- generated by render.py from small_de.json; do not edit by hand -->

### Curves

| bits | D | p | t | #E = h*r | h | r | d_K | f(pi) | crater | endomorphism | lambda | lambda^k=1, k<=1000 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 12 | -3 | 2131 | -32 | 2164 | 4 | 541 | -3 | 50 | True | zeta3 | 411 | 3 |
| 12 | -4 | 2137 | 72 | 2066 | 2 | 1033 | -4 | 29 | True | iota4 | 355 | 4 |
| 12 | -7 | 2137 | -90 | 2228 | 4 | 557 | -7 | 8 | True | +-(1+sqrt-7)/2 | 423 | 556 |
| 12 | -8 | 2179 | -2 | 2182 | 2 | 1091 | -8 | 33 | True | sqrt-2 | 1058 | None |
| 12 | -11 | 2137 | -23 | 2161 | 1 | 2161 | -11 | 27 | True | +-(1+sqrt-11)/2 | 2080 | 27 |
| 12 | 0 | 2131 | 79 | 2053 | 1 | 2053 | -2283 | 1 | None | - | - | None |
| 16 | -3 | 41047 | -229 | 41277 | 3 | 13759 | -3 | 193 | True | zeta3 | 713 | 3 |
| 16 | -4 | 41057 | -328 | 41386 | 2 | 20693 | -4 | 119 | True | iota4 | 15301 | 4 |
| 16 | -7 | 41141 | 106 | 41036 | 4 | 10259 | -7 | 148 | True | +-(1+sqrt-7)/2 | 3744 | None |
| 16 | -8 | 41227 | 406 | 40822 | 2 | 20411 | -8 | 3 | True | sqrt-2 | 13540 | None |
| 16 | -11 | 41077 | -263 | 41341 | 1 | 41341 | -11 | 93 | True | +-(1+sqrt-11)/2 | 23559 | None |
| 16 | 0 | 41047 | -115 | 41163 | 3 | 13721 | None | None | None | - | - | None |
| 20 | -3 | 549001 | 323 | 548679 | 3 | 182893 | -3 | 835 | True | zeta3 | 8542 | 3 |
| 20 | -4 | 549121 | 1480 | 547642 | 2 | 273821 | -4 | 39 | True | iota4 | 7040 | 4 |
| 20 | -7 | 549229 | -846 | 550076 | 4 | 137519 | -7 | 460 | True | +-(1+sqrt-7)/2 | 49626 | None |
| 20 | -8 | 549163 | -1478 | 550642 | 2 | 275321 | -8 | 39 | True | sqrt-2 | 261183 | None |
| 20 | -11 | 549091 | -1217 | 550309 | 1 | 550309 | -11 | 255 | True | +-(1+sqrt-11)/2 | 373345 | None |
| 20 | 0 | 548957 | 287 | 548671 | 1 | 548671 | None | None | None | - | - | None |
| 24 | -3 | 12083299 | -6907 | 12090207 | 3 | 4030069 | -3 | 457 | True | zeta3 | 1790169 | 3 |
| 24 | -4 | 12083321 | 928 | 12082394 | 2 | 6041197 | -4 | 3445 | True | iota4 | 5271361 | 4 |
| 24 | -7 | 12083689 | 5398 | 12078292 | 4 | 3019573 | -7 | 1656 | True | +-(1+sqrt-7)/2 | 1834353 | None |
| 24 | -8 | 12083299 | 2818 | 12080482 | 2 | 6040241 | -8 | 2247 | True | sqrt-2 | 626335 | None |
| 24 | -11 | 12083641 | 6715 | 12076927 | 1 | 12076927 | -11 | 543 | True | +-(1+sqrt-11)/2 | 2513252 | None |
| 24 | 0 | 12083321 | 3448 | 12079874 | 2 | 6039937 | None | None | None | - | - | None |
| 28 | -3 | 167272543 | 18827 | 167253717 | 3 | 55751239 | -3 | 10241 | True | zeta3 | 28901798 | 3 |
| 28 | -4 | 167272649 | -20864 | 167293514 | 2 | 83646757 | -4 | 7645 | True | iota4 | 5700454 | 4 |
| 28 | -7 | 167273129 | -20138 | 167293268 | 4 | 41823317 | -7 | 6136 | True | +-(1+sqrt-7)/2 | 11301019 | None |
| 28 | -8 | 167272801 | -20546 | 167293348 | 4 | 41823337 | -8 | 5556 | True | sqrt-2 | 17268309 | None |
| 28 | -11 | 167272543 | 23281 | 167249263 | 1 | 167249263 | -11 | 3399 | True | +-(1+sqrt-11)/2 | 20174231 | None |
| 28 | 0 | 167272541 | 20725 | 167251817 | 1 | 167251817 | None | None | None | - | - | None |

### D = -3: CM, End = O_K

Structure (means over repetitions). `P(dec)` = share of trials with at least one decomposition; `ops/trial` = field multiplications + inversions of the decomposition sweeps per trial; `LA` = elimination multiply-adds summed over all attempts; `rule` = repetitions that met the 90% rule; `cert` = targets whose log passed `[k]G == T`.

| bits | m | arm | points | columns | P(dec) | trials/rel | ops/trial | relations | LA ops | LA rows x cols | rule | cert |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 12 | 2 | A2 | 48 | 16 | 0.88 | 0.468 | 337 | 27.8 | 197 | 27.8x16 | 20/20 | 16/16 |
| 12 | 2 | A1 | 48 | 48 | 0.87 | 0.471 | 337 | 71.2 | 525 | 71.2x48 | 20/20 | 16/16 |
| 12 | 3 | A2 | 15 | 5 | 0.93 | 0.446 | 1710 | 7 | 32.6 | 7x5 | 20/20 | 16/16 |
| 12 | 3 | A1 | 15 | 15 | 0.82 | 0.495 | 1710 | 23.4 | 233 | 23.4x15 | 20/20 | 16/16 |
| 16 | 2 | A2 | 204 | 68 | 0.88 | 0.492 | 1429 | 108 | 894 | 108x68 | 20/20 | 16/16 |
| 16 | 2 | A1 | 204 | 204 | 0.87 | 0.487 | 1429 | 307 | 2655 | 307x204 | 20/20 | 16/16 |
| 16 | 3 | A2 | 36 | 12 | 0.78 | 0.699 | 9396 | 14.9 | 123 | 14.9x12 | 20/20 | 16/16 |
| 16 | 3 | A1 | 36 | 36 | 0.79 | 0.661 | 9396 | 39.2 | 381 | 39.2x36 | 20/20 | 16/16 |
| 20 | 2 | A2 | 741 | 247 | 0.88 | 0.494 | 5188 | 372 | 3036 | 372x247 | 10/10 | 16/16 |
| 20 | 2 | A1 | 741 | 741 | 0.88 | 0.487 | 5188 | 1113 | 10082 | 1113x741 | 10/10 | 16/16 |
| 20 | 3 | A2 | 84 | 28 | 0.76 | 0.677 | 50148 | 35.5 | 468 | 35.5x28 | 10/10 | 16/16 |
| 20 | 3 | A1 | 84 | 84 | 0.77 | 0.701 | 50148 | 106 | 2216 | 106x84 | 10/10 | 16/16 |
| 24 | 2 | A2 | 3480 | 1160 | 0.86 | 0.497 | 24361 | 1741 | 16273 | 1741x1160 | 6/6 | 16/16 |
| 24 | 2 | A1 | 3480 | 3480 | 0.87 | 0.5 | 24361 | 5222 | 50861 | 5222x3480 | 6/6 | 16/16 |
| 24 | 3 | A2 | 231 | 77 | 0.73 | 0.776 | 3.76e+05 | 77.5 | 902 | 77.5x77 | 6/6 | 16/16 |
| 24 | 3 | A1 | 231 | 231 | 0.74 | 0.75 | 3.76e+05 | 232 | 5120 | 232x231 | 6/6 | 16/16 |
| 28 | 2 | A2 | 12933 | 4311 | 0.86 | 0.501 | 90532 | 6468 | 61584 | 6468x4311 | 3/3 | 16/16 |
| 28 | 2 | A1 | 12933 | 12933 | 0.87 | 0.498 | 90532 | 19402 | 1.96e+05 | 19402x12933 | 3/3 | 16/16 |
| 28 | 3 | A2 | 552 | 184 | 0.75 | 0.74 | 2.14e+06 | 184 | 4298 | 184x184 | 3/3 | 16/16 |
| 28 | 3 | A1 | 552 | 552 | 0.76 | 0.733 | 2.14e+06 | 553 | 28195 | 553x552 | 3/3 | 16/16 |

Cost in GAE (priced group operations + w x native operations; relation phase includes the base build; single-instance = relation phase + mean target stage; `rho` = best matched variant, mean per target, with its name; `IC/rho` = single-instance total over that rho). Mean +- standard error of the repetitions is in `small_de_derived.json`.

| bits | m | arm | rel w=0 | rel w=0.1 | rel w=1 | single w=0 | single w=0.1 | single w=1 | rho w=0 | rho w=0.1 | rho w=1 | IC/rho w=0 | w=0.1 | w=1 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 12 | 2 | A2 | 505 | 1151 | 6969 | 982 | 1674 | 7906 | 96.7 (negation) | 101 (negation) | 111 (plain) | 10.2 | 16.6 | 71.4 |
| 12 | 2 | A1 | 569 | 2273 | 17602 | 1049 | 2796 | 18522 | 96.7 (negation) | 101 (negation) | 111 (plain) | 10.8 | 27.8 | 167 |
| 12 | 3 | A2 | 566 | 1242 | 7333 | 1073 | 1969 | 10030 | 86.5 (aut) | 88.2 (aut) | 100 (plain) | 12.4 | 22.3 | 100 |
| 12 | 3 | A1 | 834 | 3375 | 26245 | 1351 | 4144 | 29274 | 86.5 (aut) | 88.2 (aut) | 100 (plain) | 15.6 | 47 | 292 |
| 16 | 2 | A2 | 901 | 11095 | 1.03e+05 | 1659 | 12060 | 1.06e+05 | 184 (aut) | 191 (aut) | 254 (plain) | 9.03 | 63 | 415 |
| 16 | 2 | A1 | 1200 | 29837 | 2.88e+05 | 1970 | 30810 | 2.9e+05 | 184 (aut) | 191 (aut) | 254 (plain) | 10.7 | 161 | 1141 |
| 16 | 3 | A2 | 1504 | 13929 | 1.26e+05 | 2351 | 16110 | 1.4e+05 | 188 (aut) | 197 (aut) | 270 (aut) | 12.5 | 82 | 518 |
| 16 | 3 | A1 | 2674 | 34155 | 3.17e+05 | 3538 | 36500 | 3.33e+05 | 188 (aut) | 197 (aut) | 270 (aut) | 18.8 | 186 | 1234 |
| 20 | 2 | A2 | 1453 | 1.31e+05 | 1.3e+06 | 2372 | 1.33e+05 | 1.31e+06 | 418 (aut) | 442 (aut) | 659 (aut) | 5.67 | 301 | 1984 |
| 20 | 2 | A1 | 2545 | 3.85e+05 | 3.83e+06 | 3463 | 3.86e+05 | 3.83e+06 | 418 (aut) | 442 (aut) | 659 (aut) | 8.28 | 874 | 5818 |
| 20 | 3 | A2 | 5002 | 1.61e+05 | 1.57e+06 | 6170 | 1.72e+05 | 1.66e+06 | 396 (aut) | 418 (aut) | 615 (aut) | 15.6 | 411 | 2705 |
| 20 | 3 | A1 | 13498 | 4.93e+05 | 4.81e+06 | 14608 | 5.01e+05 | 4.88e+06 | 396 (aut) | 418 (aut) | 615 (aut) | 36.9 | 1199 | 7936 |
| 24 | 2 | A2 | 3773 | 2.73e+06 | 2.72e+07 | 4979 | 2.73e+06 | 2.73e+07 | 1268 (aut) | 1375 (aut) | 2332 (aut) | 3.93 | 1985 | 11685 |
| 24 | 2 | A1 | 9010 | 8.21e+06 | 8.21e+07 | 10221 | 8.22e+06 | 8.21e+07 | 1268 (aut) | 1375 (aut) | 2332 (aut) | 8.06 | 5978 | 35202 |
| 24 | 3 | A2 | 29110 | 2.93e+06 | 2.91e+07 | 30926 | 2.99e+06 | 2.97e+07 | 1352 (aut) | 1466 (aut) | 2501 (aut) | 22.9 | 2041 | 11859 |
| 24 | 3 | A1 | 81800 | 8.47e+06 | 8.39e+07 | 83486 | 8.52e+06 | 8.44e+07 | 1352 (aut) | 1466 (aut) | 2501 (aut) | 61.8 | 5807 | 33752 |
| 28 | 2 | A2 | 10996 | 3.78e+07 | 3.78e+08 | 12320 | 3.78e+07 | 3.78e+08 | 4116 (aut) | 4486 (aut) | 7822 (aut) | 2.99 | 8429 | 48325 |
| 28 | 2 | A1 | 30328 | 1.13e+08 | 1.13e+09 | 31655 | 1.13e+08 | 1.13e+09 | 4116 (aut) | 4486 (aut) | 7822 (aut) | 7.69 | 25093 | 1.44e+05 |
| 28 | 3 | A2 | 1.52e+05 | 3.76e+07 | 3.75e+08 | 1.55e+05 | 3.81e+07 | 3.79e+08 | 4136 (aut) | 4508 (aut) | 7861 (aut) | 37.5 | 8440 | 48229 |
| 28 | 3 | A1 | 4.49e+05 | 1.12e+08 | 1.11e+09 | 4.52e+05 | 1.12e+08 | 1.12e+09 | 4136 (aut) | 4508 (aut) | 7861 (aut) | 109 | 24873 | 1.42e+05 |

Matched rho by variant (mean GAE per target; native = canonicalisations the unit does not charge; field ops = multiplications + inversions):

| bits | m | variant | A | runs | GAE | native | field ops |
|---|---|---|---|---|---|---|---|
| 12 | 2,3 | plain | 1 | 16/16 | 111 | 0 | 585 |
| 12 | 2,3 | negation | 2 | 16/16 | 96.7 | 39.1 | 529 |
| 12 | 2,3 | aut | 6 | 16/16 | 129 | 58.2 | 811 |
| 16 | 2,3 | plain | 1 | 16/16 | 254 | 0 | 1196 |
| 16 | 2,3 | negation | 2 | 16/16 | 221 | 149 | 1070 |
| 16 | 2,3 | aut | 6 | 16/16 | 184 | 77.9 | 1159 |
| 20 | 2,3 | plain | 1 | 16/16 | 717 | 0 | 3246 |
| 20 | 2,3 | negation | 2 | 16/16 | 595 | 437 | 2762 |
| 20 | 2,3 | aut | 6 | 16/16 | 418 | 241 | 2583 |
| 24 | 2,3 | plain | 1 | 16/16 | 2552 | 0 | 10622 |
| 24 | 2,3 | negation | 2 | 16/16 | 2967 | 2792 | 12334 |
| 24 | 2,3 | aut | 6 | 16/16 | 1268 | 1064 | 7703 |
| 28 | 2,3 | plain | 1 | 16/16 | 10617 | 0 | 43343 |
| 28 | 2,3 | negation | 2 | 16/16 | 7575 | 7201 | 31208 |
| 28 | 2,3 | aut | 6 | 16/16 | 4116 | 3707 | 24854 |

### D = -4: CM, End = O_K

Structure (means over repetitions). `P(dec)` = share of trials with at least one decomposition; `ops/trial` = field multiplications + inversions of the decomposition sweeps per trial; `LA` = elimination multiply-adds summed over all attempts; `rule` = repetitions that met the 90% rule; `cert` = targets whose log passed `[k]G == T`.

| bits | m | arm | points | columns | P(dec) | trials/rel | ops/trial | relations | LA ops | LA rows x cols | rule | cert |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 12 | 2 | A2 | 46 | 23 | 0.89 | 0.476 | 323 | 37.6 | 280 | 37.6x23 | 20/20 | 16/16 |
| 12 | 2 | A1 | 46 | 46 | 0.87 | 0.504 | 323 | 71.3 | 556 | 71.3x46 | 20/20 | 16/16 |
| 12 | 3 | A2 | 14 | 7 | 0.76 | 0.572 | 1497 | 12.8 | 103 | 12.8x7 | 20/20 | 16/16 |
| 12 | 3 | A1 | 14 | 14 | 0.79 | 0.427 | 1497 | 31 | 297 | 31x14 | 20/20 | 16/16 |
| 16 | 2 | A2 | 204 | 102 | 0.88 | 0.51 | 1429 | 158 | 1302 | 158x102 | 20/20 | 16/16 |
| 16 | 2 | A1 | 204 | 204 | 0.87 | 0.509 | 1429 | 314 | 2781 | 314x204 | 20/20 | 16/16 |
| 16 | 3 | A2 | 36 | 18 | 0.81 | 0.636 | 9396 | 25.1 | 262 | 25.1x18 | 20/20 | 16/16 |
| 16 | 3 | A1 | 36 | 36 | 0.76 | 0.637 | 9396 | 48.8 | 582 | 48.8x36 | 20/20 | 16/16 |
| 20 | 2 | A2 | 742 | 371 | 0.87 | 0.501 | 5195 | 558 | 4858 | 558x371 | 10/10 | 16/16 |
| 20 | 2 | A1 | 742 | 742 | 0.87 | 0.496 | 5195 | 1114 | 10365 | 1114x742 | 10/10 | 16/16 |
| 20 | 3 | A2 | 82 | 41 | 0.67 | 0.816 | 47806 | 58.6 | 628 | 58.6x41 | 10/10 | 16/16 |
| 20 | 3 | A1 | 82 | 82 | 0.74 | 0.744 | 47806 | 94.7 | 1600 | 94.7x82 | 10/10 | 16/16 |
| 24 | 2 | A2 | 3476 | 1738 | 0.87 | 0.502 | 24333 | 2609 | 23999 | 2609x1738 | 6/6 | 16/16 |
| 24 | 2 | A1 | 3476 | 3476 | 0.87 | 0.499 | 24333 | 5215 | 51032 | 5215x3476 | 6/6 | 16/16 |
| 24 | 3 | A2 | 230 | 115 | 0.75 | 0.702 | 3.72e+05 | 135 | 3335 | 135x115 | 6/6 | 16/16 |
| 24 | 3 | A1 | 230 | 230 | 0.75 | 0.712 | 3.72e+05 | 251 | 6849 | 251x230 | 6/6 | 16/16 |
| 28 | 2 | A2 | 12936 | 6468 | 0.87 | 0.497 | 90553 | 9704 | 93966 | 9704x6468 | 3/3 | 16/16 |
| 28 | 2 | A1 | 12936 | 12936 | 0.86 | 0.5 | 90553 | 19405 | 1.89e+05 | 19405x12936 | 3/3 | 16/16 |
| 28 | 3 | A2 | 552 | 276 | 0.75 | 0.708 | 2.14e+06 | 276 | 7461 | 276x276 | 3/3 | 16/16 |
| 28 | 3 | A1 | 552 | 552 | 0.74 | 0.748 | 2.14e+06 | 553 | 24676 | 553x552 | 3/3 | 16/16 |

Cost in GAE (priced group operations + w x native operations; relation phase includes the base build; single-instance = relation phase + mean target stage; `rho` = best matched variant, mean per target, with its name; `IC/rho` = single-instance total over that rho). Mean +- standard error of the repetitions is in `small_de_derived.json`.

| bits | m | arm | rel w=0 | rel w=0.1 | rel w=1 | single w=0 | single w=0.1 | single w=1 | rho w=0 | rho w=0.1 | rho w=1 | IC/rho w=0 | w=0.1 | w=1 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 12 | 2 | A2 | 498 | 1619 | 11705 | 956 | 2124 | 12637 | 98.2 (negation) | 102 (negation) | 115 (plain) | 9.74 | 20.8 | 110 |
| 12 | 2 | A1 | 554 | 2777 | 22780 | 1012 | 3282 | 23709 | 98.2 (negation) | 102 (negation) | 115 (plain) | 10.3 | 32.2 | 206 |
| 12 | 3 | A2 | 639 | 1952 | 13768 | 1134 | 2699 | 16780 | 102 (aut) | 105 (aut) | 124 (plain) | 11.2 | 25.8 | 136 |
| 12 | 3 | A1 | 812 | 3289 | 25577 | 1306 | 4035 | 28588 | 102 (aut) | 105 (aut) | 124 (plain) | 12.8 | 38.6 | 231 |
| 16 | 2 | A2 | 925 | 17361 | 1.65e+05 | 1635 | 18291 | 1.68e+05 | 193 (aut) | 203 (aut) | 297 (aut) | 8.48 | 90 | 566 |
| 16 | 2 | A1 | 1164 | 33706 | 3.27e+05 | 1878 | 34606 | 3.29e+05 | 193 (aut) | 203 (aut) | 297 (aut) | 9.74 | 170 | 1109 |
| 16 | 3 | A2 | 1798 | 20074 | 1.85e+05 | 2604 | 22538 | 2.02e+05 | 201 (aut) | 213 (aut) | 314 (aut) | 12.9 | 106 | 643 |
| 16 | 3 | A1 | 2871 | 38783 | 3.62e+05 | 3694 | 41565 | 3.82e+05 | 201 (aut) | 213 (aut) | 314 (aut) | 18.3 | 196 | 1218 |
| 20 | 2 | A2 | 1736 | 1.99e+05 | 1.97e+06 | 2664 | 2.01e+05 | 1.98e+06 | 629 (aut) | 673 (aut) | 799 (plain) | 4.24 | 298 | 2481 |
| 20 | 2 | A1 | 2569 | 3.93e+05 | 3.9e+06 | 3504 | 3.94e+05 | 3.91e+06 | 629 (aut) | 673 (aut) | 799 (plain) | 5.57 | 586 | 4896 |
| 20 | 3 | A2 | 7394 | 2.48e+05 | 2.41e+06 | 8565 | 2.58e+05 | 2.5e+06 | 502 (aut) | 533 (aut) | 816 (aut) | 17.1 | 484 | 3066 |
| 20 | 3 | A1 | 12613 | 4.47e+05 | 4.36e+06 | 13748 | 4.56e+05 | 4.43e+06 | 502 (aut) | 533 (aut) | 816 (aut) | 27.4 | 855 | 5432 |
| 24 | 2 | A2 | 5050 | 4.14e+06 | 4.14e+07 | 6221 | 4.15e+06 | 4.14e+07 | 2044 (aut) | 2209 (aut) | 3064 (plain) | 3.04 | 1878 | 13521 |
| 24 | 2 | A1 | 8944 | 8.24e+06 | 8.23e+07 | 10121 | 8.24e+06 | 8.23e+07 | 2044 (aut) | 2209 (aut) | 3064 (plain) | 4.95 | 3731 | 26868 |
| 24 | 3 | A2 | 45442 | 4.64e+06 | 4.59e+07 | 47223 | 4.7e+06 | 4.65e+07 | 2178 (aut) | 2355 (aut) | 3786 (plain) | 21.7 | 1994 | 12294 |
| 24 | 3 | A1 | 83750 | 8.64e+06 | 8.57e+07 | 85470 | 8.7e+06 | 8.62e+07 | 2178 (aut) | 2355 (aut) | 3786 (plain) | 39.3 | 3692 | 22763 |
| 28 | 2 | A2 | 15841 | 5.64e+07 | 5.64e+08 | 17199 | 5.64e+07 | 5.64e+08 | 6301 (aut) | 6890 (aut) | 12189 (aut) | 2.73 | 8183 | 46247 |
| 28 | 2 | A1 | 30416 | 1.13e+08 | 1.13e+09 | 31779 | 1.13e+08 | 1.13e+09 | 6301 (aut) | 6890 (aut) | 12189 (aut) | 5.04 | 16444 | 92933 |
| 28 | 3 | A2 | 2.18e+05 | 5.4e+07 | 5.38e+08 | 2.21e+05 | 5.44e+07 | 5.42e+08 | 5977 (aut) | 6533 (aut) | 11542 (aut) | 37 | 8324 | 46947 |
| 28 | 3 | A1 | 4.59e+05 | 1.14e+08 | 1.14e+09 | 4.62e+05 | 1.15e+08 | 1.14e+09 | 5977 (aut) | 6533 (aut) | 11542 (aut) | 77.3 | 17529 | 98865 |

Matched rho by variant (mean GAE per target; native = canonicalisations the unit does not charge; field ops = multiplications + inversions):

| bits | m | variant | A | runs | GAE | native | field ops |
|---|---|---|---|---|---|---|---|
| 12 | 2,3 | plain | 1 | 16/16 | 115 | 0 | 609 |
| 12 | 2,3 | negation | 2 | 16/16 | 98.2 | 38.1 | 542 |
| 12 | 2,3 | aut | 4 | 16/16 | 99.9 | 27.9 | 608 |
| 16 | 2,3 | plain | 1 | 16/16 | 353 | 0 | 1590 |
| 16 | 2,3 | negation | 2 | 16/16 | 274 | 200 | 1284 |
| 16 | 2,3 | aut | 4 | 16/16 | 193 | 104 | 1103 |
| 20 | 2,3 | plain | 1 | 16/16 | 799 | 0 | 3575 |
| 20 | 2,3 | negation | 2 | 16/16 | 655 | 493 | 3009 |
| 20 | 2,3 | aut | 4 | 16/16 | 629 | 444 | 3413 |
| 24 | 2,3 | plain | 1 | 16/16 | 3064 | 0 | 13107 |
| 24 | 2,3 | negation | 2 | 16/16 | 3061 | 2694 | 13106 |
| 24 | 2,3 | aut | 4 | 16/16 | 2044 | 1647 | 10753 |
| 28 | 2,3 | plain | 1 | 16/16 | 13614 | 0 | 55337 |
| 28 | 2,3 | negation | 2 | 16/16 | 9341 | 8963 | 38281 |
| 28 | 2,3 | aut | 4 | 16/16 | 6301 | 5887 | 32066 |

### D = -7: CM, End = O_K

Structure (means over repetitions). `P(dec)` = share of trials with at least one decomposition; `ops/trial` = field multiplications + inversions of the decomposition sweeps per trial; `LA` = elimination multiply-adds summed over all attempts; `rule` = repetitions that met the 90% rule; `cert` = targets whose log passed `[k]G == T`.

| bits | m | arm | points | columns | P(dec) | trials/rel | ops/trial | relations | LA ops | LA rows x cols | rule | cert |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 12 | 2 | A2 | 43 | 12 | 0.69 | 0.65 | 302 | 18.3 | 108 | 18.3x12 | 20/20 | 16/16 |
| 12 | 2 | A1 | 43 | 43 | 0.87 | 0.564 | 302 | 64.8 | 471 | 64.8x43 | 20/20 | 16/16 |
| 12 | 3 | A2 | 16 | 4 | 0.40 | 0.396 | 1934 | 11.3 | 52.8 | 11.3x4 | 20/20 | 16/16 |
| 12 | 3 | A1 | 16 | 16 | 0.93 | 0.383 | 1935 | 22.4 | 199 | 22.4x16 | 20/20 | 16/16 |
| 16 | 2 | A2 | 203 | 51 | 0.78 | 0.507 | 1422 | 81.7 | 646 | 81.7x51 | 20/20 | 16/16 |
| 16 | 2 | A1 | 203 | 203 | 0.86 | 0.499 | 1422 | 306 | 2596 | 306x203 | 20/20 | 16/16 |
| 16 | 3 | A2 | 36 | 9 | 0.79 | 0.639 | 9396 | 13 | 120 | 13x9 | 20/20 | 16/16 |
| 16 | 3 | A1 | 36 | 36 | 0.81 | 0.639 | 9396 | 44.2 | 555 | 44.2x36 | 20/20 | 16/16 |
| 20 | 2 | A2 | 744 | 186 | 0.78 | 0.485 | 5209 | 294 | 2577 | 294x186 | 10/10 | 16/16 |
| 20 | 2 | A1 | 744 | 744 | 0.87 | 0.5 | 5209 | 1118 | 10224 | 1118x744 | 10/10 | 16/16 |
| 20 | 3 | A2 | 84 | 21 | 0.81 | 0.663 | 50148 | 27.1 | 288 | 27.1x21 | 10/10 | 16/16 |
| 20 | 3 | A1 | 84 | 84 | 0.80 | 0.685 | 50148 | 89.3 | 1180 | 89.3x84 | 10/10 | 16/16 |
| 24 | 2 | A2 | 3476 | 869 | 0.77 | 0.492 | 24333 | 1307 | 12118 | 1307x869 | 6/6 | 16/16 |
| 24 | 2 | A1 | 3476 | 3476 | 0.86 | 0.5 | 24333 | 5215 | 49037 | 5215x3476 | 6/6 | 16/16 |
| 24 | 3 | A2 | 232 | 58 | 0.65 | 0.779 | 3.79e+05 | 63.7 | 989 | 63.7x58 | 6/6 | 16/16 |
| 24 | 3 | A1 | 232 | 232 | 0.76 | 0.704 | 3.79e+05 | 233 | 5292 | 233x232 | 6/6 | 16/16 |
| 28 | 2 | A2 | 12936 | 3234 | 0.77 | 0.491 | 90553 | 4852 | 46558 | 4852x3234 | 3/3 | 16/16 |
| 28 | 2 | A1 | 12936 | 12936 | 0.87 | 0.5 | 90553 | 19406 | 1.97e+05 | 19406x12936 | 3/3 | 16/16 |
| 28 | 3 | A2 | 552 | 138 | 0.68 | 0.76 | 2.14e+06 | 163 | 5038 | 163x138 | 3/3 | 16/16 |
| 28 | 3 | A1 | 552 | 552 | 0.71 | 0.776 | 2.14e+06 | 554 | 29137 | 554x552 | 3/3 | 16/16 |

Cost in GAE (priced group operations + w x native operations; relation phase includes the base build; single-instance = relation phase + mean target stage; `rho` = best matched variant, mean per target, with its name; `IC/rho` = single-instance total over that rho). Mean +- standard error of the repetitions is in `small_de_derived.json`.

| bits | m | arm | rel w=0 | rel w=0.1 | rel w=1 | single w=0 | single w=0.1 | single w=1 | rho w=0 | rho w=0.1 | rho w=1 | IC/rho w=0 | w=0.1 | w=1 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 12 | 2 | A2 | 538 | 1180 | 6954 | 1020 | 1705 | 7872 | 99.1 (negation) | 103 (negation) | 128 (plain) | 10.3 | 16.5 | 61.5 |
| 12 | 2 | A1 | 567 | 2652 | 21422 | 1041 | 3170 | 22325 | 99.1 (negation) | 103 (negation) | 128 (plain) | 10.5 | 30.7 | 174 |
| 12 | 3 | A2 | 628 | 1717 | 11524 | 1241 | 3307 | 21902 | 98.6 (plain) | 98.6 (plain) | 98.6 (plain) | 12.6 | 33.6 | 222 |
| 12 | 3 | A1 | 757 | 3041 | 23595 | 1270 | 3802 | 26586 | 98.6 (plain) | 98.6 (plain) | 98.6 (plain) | 12.9 | 38.6 | 270 |
| 16 | 2 | A2 | 965 | 9295 | 84260 | 1621 | 10137 | 86781 | 248 (plain) | 248 (plain) | 248 (plain) | 6.53 | 40.8 | 349 |
| 16 | 2 | A1 | 1103 | 31961 | 3.1e+05 | 1760 | 32830 | 3.12e+05 | 248 (plain) | 248 (plain) | 248 (plain) | 7.08 | 132 | 1257 |
| 16 | 3 | A2 | 1295 | 11380 | 1.02e+05 | 2038 | 13706 | 1.19e+05 | 234 (negation) | 250 (negation) | 261 (plain) | 8.7 | 54.8 | 455 |
| 16 | 3 | A1 | 2725 | 36969 | 3.45e+05 | 3463 | 38987 | 3.59e+05 | 234 (negation) | 250 (negation) | 261 (plain) | 14.8 | 156 | 1374 |
| 20 | 2 | A2 | 2055 | 1.01e+05 | 9.94e+05 | 2953 | 1.03e+05 | 1e+06 | 523 (negation) | 560 (negation) | 691 (plain) | 5.65 | 184 | 1452 |
| 20 | 2 | A1 | 2541 | 3.9e+05 | 3.88e+06 | 3432 | 3.91e+05 | 3.88e+06 | 523 (negation) | 560 (negation) | 691 (plain) | 6.56 | 699 | 5615 |
| 20 | 3 | A2 | 4007 | 1.2e+05 | 1.16e+06 | 5133 | 1.3e+05 | 1.25e+06 | 556 (negation) | 596 (negation) | 666 (plain) | 9.24 | 218 | 1878 |
| 20 | 3 | A1 | 11328 | 4.08e+05 | 3.98e+06 | 12420 | 4.16e+05 | 4.05e+06 | 556 (negation) | 596 (negation) | 666 (plain) | 22.3 | 698 | 6077 |
| 24 | 2 | A2 | 6510 | 2.07e+06 | 2.06e+07 | 7629 | 2.07e+06 | 2.06e+07 | 1479 (negation) | 1609 (negation) | 2681 (plain) | 5.16 | 1286 | 7695 |
| 24 | 2 | A1 | 8921 | 8.36e+06 | 8.35e+07 | 10038 | 8.36e+06 | 8.35e+07 | 1479 (negation) | 1609 (negation) | 2681 (plain) | 6.79 | 5197 | 31162 |
| 24 | 3 | A2 | 24166 | 2.41e+06 | 2.39e+07 | 25899 | 2.48e+06 | 2.45e+07 | 2037 (negation) | 2224 (negation) | 3002 (plain) | 12.7 | 1114 | 8174 |
| 24 | 3 | A1 | 77735 | 8.09e+06 | 8.02e+07 | 79371 | 8.15e+06 | 8.07e+07 | 2037 (negation) | 2224 (negation) | 3002 (plain) | 39 | 3663 | 26894 |
| 28 | 2 | A2 | 21483 | 2.79e+07 | 2.79e+08 | 22824 | 2.79e+07 | 2.79e+08 | 6432 (negation) | 7038 (negation) | 11292 (plain) | 3.55 | 3963 | 24683 |
| 28 | 2 | A1 | 30429 | 1.13e+08 | 1.13e+09 | 31767 | 1.14e+08 | 1.13e+09 | 6432 (negation) | 7038 (negation) | 11292 (plain) | 4.94 | 16129 | 1e+05 |
| 28 | 3 | A2 | 1.38e+05 | 3.4e+07 | 3.39e+08 | 1.41e+05 | 3.44e+07 | 3.42e+08 | 7791 (plain) | 7791 (plain) | 7791 (plain) | 18.1 | 4410 | 43935 |
| 28 | 3 | A1 | 4.77e+05 | 1.19e+08 | 1.18e+09 | 4.8e+05 | 1.19e+08 | 1.19e+09 | 7791 (plain) | 7791 (plain) | 7791 (plain) | 61.6 | 15273 | 1.52e+05 |

Matched rho by variant (mean GAE per target; native = canonicalisations the unit does not charge; field ops = multiplications + inversions):

| bits | m | variant | A | runs | GAE | native | field ops |
|---|---|---|---|---|---|---|---|
| 12 | 2,3 | plain | 1 | 16/16 | 128 | 0 | 659 |
| 12 | 2,3 | negation | 2 | 16/16 | 99.1 | 39.9 | 540 |
| 16 | 2,3 | plain | 1 | 16/16 | 248 | 0 | 1168 |
| 16 | 2,3 | negation | 2 | 16/16 | 284 | 212 | 1323 |
| 20 | 2,3 | plain | 1 | 16/16 | 691 | 0 | 3135 |
| 20 | 2,3 | negation | 2 | 16/16 | 523 | 367 | 2465 |
| 24 | 2,3 | plain | 1 | 16/16 | 2681 | 0 | 11136 |
| 24 | 2,3 | negation | 2 | 16/16 | 1479 | 1305 | 6349 |
| 28 | 2,3 | plain | 1 | 16/16 | 11292 | 0 | 46041 |
| 28 | 2,3 | negation | 2 | 16/16 | 6432 | 6056 | 26629 |

### D = -8: CM, End = O_K

Structure (means over repetitions). `P(dec)` = share of trials with at least one decomposition; `ops/trial` = field multiplications + inversions of the decomposition sweeps per trial; `LA` = elimination multiply-adds summed over all attempts; `rule` = repetitions that met the 90% rule; `cert` = targets whose log passed `[k]G == T`.

| bits | m | arm | points | columns | P(dec) | trials/rel | ops/trial | relations | LA ops | LA rows x cols | rule | cert |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 12 | 2 | A2 | 47 | 12 | 0.79 | 0.556 | 330 | 16.6 | 82.5 | 16.6x12 | 20/20 | 16/16 |
| 12 | 2 | A1 | 47 | 47 | 0.89 | 0.474 | 330 | 69.2 | 508 | 69.2x47 | 20/20 | 16/16 |
| 12 | 3 | A2 | 16 | 4 | 0.90 | 0.4 | 1936 | 8.45 | 40.5 | 8.45x4 | 20/20 | 16/16 |
| 12 | 3 | A1 | 16 | 16 | 0.96 | 0.335 | 1934 | 21.9 | 201 | 21.9x16 | 20/20 | 16/16 |
| 16 | 2 | A2 | 203 | 51 | 0.78 | 0.503 | 1422 | 80.5 | 618 | 80.5x51 | 20/20 | 16/16 |
| 16 | 2 | A1 | 203 | 203 | 0.87 | 0.491 | 1422 | 313 | 2773 | 313x203 | 20/20 | 16/16 |
| 16 | 3 | A2 | 36 | 9 | 0.72 | 0.792 | 9396 | 11.5 | 85.3 | 11.5x9 | 20/20 | 16/16 |
| 16 | 3 | A1 | 36 | 36 | 0.79 | 0.657 | 9396 | 44.5 | 529 | 44.5x36 | 20/20 | 16/16 |
| 20 | 2 | A2 | 742 | 186 | 0.76 | 0.506 | 5195 | 280 | 2422 | 280x186 | 10/10 | 16/16 |
| 20 | 2 | A1 | 742 | 742 | 0.86 | 0.498 | 5195 | 1115 | 10250 | 1115x742 | 10/10 | 16/16 |
| 20 | 3 | A2 | 84 | 21 | 0.74 | 0.637 | 50148 | 37.8 | 365 | 37.8x21 | 10/10 | 16/16 |
| 20 | 3 | A1 | 84 | 84 | 0.74 | 0.669 | 50147 | 97 | 1498 | 97x84 | 10/10 | 16/16 |
| 24 | 2 | A2 | 3474 | 869 | 0.77 | 0.496 | 24319 | 1305 | 11833 | 1305x869 | 6/6 | 16/16 |
| 24 | 2 | A1 | 3474 | 3474 | 0.87 | 0.501 | 24319 | 5212 | 49436 | 5212x3474 | 6/6 | 16/16 |
| 24 | 3 | A2 | 232 | 58 | 0.67 | 0.749 | 3.79e+05 | 64.2 | 968 | 64.2x58 | 6/6 | 16/16 |
| 24 | 3 | A1 | 232 | 232 | 0.75 | 0.722 | 3.79e+05 | 272 | 10431 | 272x232 | 6/6 | 16/16 |
| 28 | 2 | A2 | 12936 | 3234 | 0.76 | 0.514 | 90553 | 4852 | 47050 | 4852x3234 | 3/3 | 16/16 |
| 28 | 2 | A1 | 12936 | 12936 | 0.87 | 0.496 | 90553 | 19405 | 2.02e+05 | 19405x12936 | 3/3 | 16/16 |
| 28 | 3 | A2 | 552 | 138 | 0.69 | 0.817 | 2.14e+06 | 207 | 10170 | 207x138 | 3/3 | 16/16 |
| 28 | 3 | A1 | 552 | 552 | 0.72 | 0.779 | 2.14e+06 | 553 | 27390 | 553x552 | 3/3 | 16/16 |

Cost in GAE (priced group operations + w x native operations; relation phase includes the base build; single-instance = relation phase + mean target stage; `rho` = best matched variant, mean per target, with its name; `IC/rho` = single-instance total over that rho). Mean +- standard error of the repetitions is in `small_de_derived.json`.

| bits | m | arm | rel w=0 | rel w=0.1 | rel w=1 | single w=0 | single w=0.1 | single w=1 | rho w=0 | rho w=0.1 | rho w=1 | IC/rho w=0 | w=0.1 | w=1 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 12 | 2 | A2 | 532 | 996 | 5172 | 1008 | 1522 | 6152 | 110 (negation) | 114 (negation) | 128 (plain) | 9.21 | 13.3 | 48 |
| 12 | 2 | A1 | 567 | 2181 | 16700 | 1043 | 2707 | 17683 | 110 (negation) | 114 (negation) | 128 (plain) | 9.52 | 23.7 | 138 |
| 12 | 3 | A2 | 580 | 1306 | 7843 | 1110 | 2224 | 12245 | 111 (negation) | 113 (plain) | 113 (plain) | 9.99 | 19.6 | 108 |
| 12 | 3 | A1 | 721 | 2606 | 19571 | 1221 | 3324 | 22260 | 111 (negation) | 113 (plain) | 113 (plain) | 11 | 29.3 | 196 |
| 16 | 2 | A2 | 1079 | 8834 | 78625 | 1859 | 9882 | 82092 | 275 (negation) | 295 (negation) | 371 (plain) | 6.77 | 33.5 | 222 |
| 16 | 2 | A1 | 1222 | 30562 | 2.95e+05 | 2001 | 31546 | 2.97e+05 | 275 (negation) | 295 (negation) | 371 (plain) | 7.29 | 107 | 803 |
| 16 | 3 | A2 | 1454 | 12317 | 1.1e+05 | 2342 | 15015 | 1.29e+05 | 256 (negation) | 274 (negation) | 305 (plain) | 9.15 | 54.8 | 423 |
| 16 | 3 | A1 | 2851 | 37018 | 3.45e+05 | 3730 | 39466 | 3.61e+05 | 256 (negation) | 274 (negation) | 305 (plain) | 14.6 | 144 | 1184 |
| 20 | 2 | A2 | 2069 | 98307 | 9.64e+05 | 3001 | 1e+05 | 9.73e+05 | 717 (negation) | 773 (negation) | 866 (plain) | 4.18 | 129 | 1124 |
| 20 | 2 | A1 | 2576 | 3.78e+05 | 3.76e+06 | 3505 | 3.8e+05 | 3.77e+06 | 717 (negation) | 773 (negation) | 866 (plain) | 4.89 | 492 | 4352 |
| 20 | 3 | A2 | 4112 | 1.22e+05 | 1.18e+06 | 5316 | 1.34e+05 | 1.29e+06 | 789 (negation) | 852 (negation) | 1008 (plain) | 6.74 | 157 | 1280 |
| 20 | 3 | A1 | 12018 | 4.33e+05 | 4.22e+06 | 13180 | 4.42e+05 | 4.3e+06 | 789 (negation) | 852 (negation) | 1008 (plain) | 16.7 | 519 | 4267 |
| 24 | 2 | A2 | 6527 | 2.04e+06 | 2.03e+07 | 7665 | 2.04e+06 | 2.04e+07 | 2460 (negation) | 2669 (negation) | 4415 (plain) | 3.12 | 766 | 4612 |
| 24 | 2 | A1 | 8915 | 8.2e+06 | 8.19e+07 | 10048 | 8.2e+06 | 8.19e+07 | 2460 (negation) | 2669 (negation) | 4415 (plain) | 4.09 | 3074 | 18559 |
| 24 | 3 | A2 | 23876 | 2.38e+06 | 2.36e+07 | 25674 | 2.44e+06 | 2.42e+07 | 3096 (negation) | 3369 (negation) | 3983 (plain) | 8.29 | 725 | 6077 |
| 24 | 3 | A1 | 92509 | 9.64e+06 | 9.56e+07 | 94514 | 9.72e+06 | 9.64e+07 | 3096 (negation) | 3369 (negation) | 3983 (plain) | 30.5 | 2887 | 24204 |
| 28 | 2 | A2 | 21574 | 2.92e+07 | 2.92e+08 | 22916 | 2.92e+07 | 2.92e+08 | 5523 (negation) | 6037 (negation) | 10669 (negation) | 4.15 | 4836 | 27350 |
| 28 | 2 | A1 | 30329 | 1.13e+08 | 1.13e+09 | 31671 | 1.13e+08 | 1.13e+09 | 5523 (negation) | 6037 (negation) | 10669 (negation) | 5.73 | 18679 | 1.06e+05 |
| 28 | 3 | A2 | 1.89e+05 | 4.67e+07 | 4.66e+08 | 1.92e+05 | 4.72e+07 | 4.7e+08 | 5703 (negation) | 6236 (negation) | 8802 (plain) | 33.7 | 7561 | 53372 |
| 28 | 3 | A1 | 4.78e+05 | 1.19e+08 | 1.18e+09 | 4.81e+05 | 1.19e+08 | 1.19e+09 | 5703 (negation) | 6236 (negation) | 8802 (plain) | 84.4 | 19136 | 1.35e+05 |

Matched rho by variant (mean GAE per target; native = canonicalisations the unit does not charge; field ops = multiplications + inversions):

| bits | m | variant | A | runs | GAE | native | field ops |
|---|---|---|---|---|---|---|---|
| 12 | 2,3 | plain | 1 | 16/16 | 128 | 0 | 661 |
| 12 | 2,3 | negation | 2 | 16/16 | 110 | 47.8 | 586 |
| 16 | 2,3 | plain | 1 | 16/16 | 371 | 0 | 1666 |
| 16 | 2,3 | negation | 2 | 16/16 | 275 | 200 | 1292 |
| 20 | 2,3 | plain | 1 | 16/16 | 866 | 0 | 3846 |
| 20 | 2,3 | negation | 2 | 16/16 | 717 | 557 | 3255 |
| 24 | 2,3 | plain | 1 | 16/16 | 4415 | 0 | 18516 |
| 24 | 2,3 | negation | 2 | 16/16 | 2460 | 2092 | 10702 |
| 28 | 2,3 | plain | 1 | 16/16 | 11053 | 0 | 45085 |
| 28 | 2,3 | negation | 2 | 16/16 | 5523 | 5146 | 22988 |

### D = -11: CM, End = O_K

Structure (means over repetitions). `P(dec)` = share of trials with at least one decomposition; `ops/trial` = field multiplications + inversions of the decomposition sweeps per trial; `LA` = elimination multiply-adds summed over all attempts; `rule` = repetitions that met the 90% rule; `cert` = targets whose log passed `[k]G == T`.

| bits | m | arm | points | columns | P(dec) | trials/rel | ops/trial | relations | LA ops | LA rows x cols | rule | cert |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 12 | 2 | A2 | 48 | 12 | 0.89 | 0.46 | 337 | 15.8 | 78.9 | 15.8x12 | 20/20 | 16/16 |
| 12 | 2 | A1 | 48 | 48 | 0.91 | 0.463 | 337 | 73.8 | 565 | 73.8x48 | 20/20 | 16/16 |
| 12 | 3 | A2 | 16 | 4 | 1.00 | 0.353 | 1935 | 7.7 | 49.8 | 7.7x4 | 20/20 | 16/16 |
| 12 | 3 | A1 | 16 | 16 | 0.93 | 0.369 | 1934 | 23.4 | 222 | 23.4x16 | 20/20 | 16/16 |
| 16 | 2 | A2 | 204 | 51 | 0.90 | 0.477 | 1429 | 80.5 | 649 | 80.5x51 | 20/20 | 16/16 |
| 16 | 2 | A1 | 204 | 204 | 0.87 | 0.495 | 1429 | 307 | 2676 | 307x204 | 20/20 | 16/16 |
| 16 | 3 | A2 | 36 | 9 | 0.79 | 0.66 | 9396 | 10.3 | 62.5 | 10.3x9 | 20/20 | 16/16 |
| 16 | 3 | A1 | 36 | 36 | 0.79 | 0.644 | 9396 | 46 | 464 | 46x36 | 20/20 | 16/16 |
| 20 | 2 | A2 | 744 | 186 | 0.88 | 0.498 | 5209 | 280 | 2379 | 280x186 | 10/10 | 16/16 |
| 20 | 2 | A1 | 744 | 744 | 0.87 | 0.502 | 5209 | 1117 | 10258 | 1117x744 | 10/10 | 16/16 |
| 20 | 3 | A2 | 84 | 21 | 0.77 | 0.651 | 50148 | 22.1 | 164 | 22.1x21 | 10/10 | 16/16 |
| 20 | 3 | A1 | 84 | 84 | 0.77 | 0.667 | 50148 | 88.7 | 1201 | 88.7x84 | 10/10 | 16/16 |
| 24 | 2 | A2 | 3473 | 869 | 0.88 | 0.495 | 24312 | 1304 | 11865 | 1304x869 | 6/6 | 16/16 |
| 24 | 2 | A1 | 3473 | 3473 | 0.86 | 0.499 | 24312 | 5210 | 50790 | 5210x3473 | 6/6 | 16/16 |
| 24 | 3 | A2 | 232 | 58 | 0.72 | 0.749 | 3.79e+05 | 63.7 | 964 | 63.7x58 | 6/6 | 16/16 |
| 24 | 3 | A1 | 232 | 232 | 0.72 | 0.764 | 3.79e+05 | 252 | 7650 | 252x232 | 6/6 | 16/16 |
| 28 | 2 | A2 | 12935 | 3234 | 0.87 | 0.492 | 90546 | 4851 | 45878 | 4851x3234 | 3/3 | 16/16 |
| 28 | 2 | A1 | 12935 | 12935 | 0.86 | 0.499 | 90546 | 19403 | 1.96e+05 | 19403x12935 | 3/3 | 16/16 |
| 28 | 3 | A2 | 552 | 138 | 0.71 | 0.777 | 2.14e+06 | 162 | 4360 | 162x138 | 3/3 | 16/16 |
| 28 | 3 | A1 | 552 | 552 | 0.72 | 0.755 | 2.14e+06 | 645 | 54561 | 645x552 | 3/3 | 16/16 |

Cost in GAE (priced group operations + w x native operations; relation phase includes the base build; single-instance = relation phase + mean target stage; `rho` = best matched variant, mean per target, with its name; `IC/rho` = single-instance total over that rho). Mean +- standard error of the repetitions is in `small_de_derived.json`.

| bits | m | arm | rel w=0 | rel w=0.1 | rel w=1 | single w=0 | single w=0.1 | single w=1 | rho w=0 | rho w=0.1 | rho w=1 | IC/rho w=0 | w=0.1 | w=1 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 12 | 2 | A2 | 351 | 832 | 5158 | 586 | 1115 | 5880 | 146 (plain) | 146 (plain) | 146 (plain) | 4 | 7.62 | 40.2 |
| 12 | 2 | A1 | 334 | 2556 | 22557 | 567 | 2831 | 23205 | 146 (plain) | 146 (plain) | 146 (plain) | 3.87 | 19.3 | 158 |
| 12 | 3 | A2 | 343 | 923 | 6138 | 608 | 1435 | 8883 | 160 (plain) | 160 (plain) | 160 (plain) | 3.81 | 9 | 55.7 |
| 12 | 3 | A1 | 525 | 2835 | 23627 | 794 | 3351 | 26369 | 160 (plain) | 160 (plain) | 160 (plain) | 4.98 | 21 | 165 |
| 16 | 2 | A2 | 865 | 8761 | 79825 | 1205 | 9315 | 82304 | 427 (negation) | 454 (negation) | 460 (plain) | 2.83 | 20.5 | 179 |
| 16 | 2 | A1 | 799 | 31762 | 3.1e+05 | 1138 | 32300 | 3.13e+05 | 427 (negation) | 454 (negation) | 460 (plain) | 2.67 | 71.1 | 679 |
| 16 | 3 | A2 | 907 | 9140 | 83233 | 1344 | 11234 | 1e+05 | 443 (negation) | 473 (negation) | 478 (plain) | 3.03 | 23.8 | 210 |
| 16 | 3 | A1 | 2424 | 36626 | 3.44e+05 | 2848 | 38330 | 3.58e+05 | 443 (negation) | 473 (negation) | 478 (plain) | 6.43 | 81.1 | 748 |
| 20 | 2 | A2 | 2344 | 97047 | 9.49e+05 | 2783 | 98269 | 9.58e+05 | 923 (negation) | 999 (negation) | 1215 (plain) | 3.01 | 98.3 | 788 |
| 20 | 2 | A1 | 2115 | 3.83e+05 | 3.81e+06 | 2550 | 3.84e+05 | 3.82e+06 | 923 (negation) | 999 (negation) | 1215 (plain) | 2.76 | 385 | 3145 |
| 20 | 3 | A2 | 3048 | 95317 | 9.26e+05 | 3705 | 1.04e+05 | 1.01e+06 | 1184 (negation) | 1253 (plain) | 1253 (plain) | 3.13 | 83.4 | 807 |
| 20 | 3 | A1 | 10544 | 3.93e+05 | 3.84e+06 | 11230 | 4.03e+05 | 3.93e+06 | 1184 (negation) | 1253 (plain) | 1253 (plain) | 9.48 | 322 | 3135 |
| 24 | 2 | A2 | 9451 | 2.06e+06 | 2.05e+07 | 10006 | 2.07e+06 | 2.06e+07 | 3674 (negation) | 4005 (negation) | 4398 (plain) | 2.72 | 516 | 4677 |
| 24 | 2 | A1 | 8350 | 8.27e+06 | 8.26e+07 | 8901 | 8.28e+06 | 8.27e+07 | 3674 (negation) | 4005 (negation) | 4398 (plain) | 2.42 | 2066 | 18797 |
| 24 | 3 | A2 | 23314 | 2.35e+06 | 2.33e+07 | 24506 | 2.42e+06 | 2.4e+07 | 3751 (negation) | 4090 (negation) | 4907 (plain) | 6.53 | 591 | 4881 |
| 24 | 3 | A1 | 90151 | 9.46e+06 | 9.38e+07 | 91296 | 9.52e+06 | 9.43e+07 | 3751 (negation) | 4090 (negation) | 4907 (plain) | 24.3 | 2327 | 19224 |
| 28 | 2 | A2 | 33752 | 2.79e+07 | 2.78e+08 | 34400 | 2.79e+07 | 2.78e+08 | 11260 (negation) | 12347 (negation) | 16360 (plain) | 3.06 | 2258 | 17022 |
| 28 | 2 | A1 | 29740 | 1.13e+08 | 1.13e+09 | 30390 | 1.13e+08 | 1.13e+09 | 11260 (negation) | 12347 (negation) | 16360 (plain) | 2.7 | 9150 | 69038 |
| 28 | 3 | A2 | 1.43e+05 | 3.52e+07 | 3.51e+08 | 1.45e+05 | 3.56e+07 | 3.54e+08 | 13040 (negation) | 14195 (plain) | 14195 (plain) | 11.1 | 2507 | 24974 |
| 28 | 3 | A1 | 5.39e+05 | 1.34e+08 | 1.34e+09 | 5.41e+05 | 1.35e+08 | 1.34e+09 | 13040 (negation) | 14195 (plain) | 14195 (plain) | 41.5 | 9491 | 94565 |

Matched rho by variant (mean GAE per target; native = canonicalisations the unit does not charge; field ops = multiplications + inversions):

| bits | m | variant | A | runs | GAE | native | field ops |
|---|---|---|---|---|---|---|---|
| 12 | 2,3 | plain | 1 | 16/16 | 146 | 0 | 743 |
| 12 | 2,3 | negation | 2 | 16/16 | 178 | 113 | 876 |
| 16 | 2,3 | plain | 1 | 16/16 | 460 | 0 | 2196 |
| 16 | 2,3 | negation | 2 | 16/16 | 427 | 277 | 2066 |
| 20 | 2,3 | plain | 1 | 16/16 | 1215 | 0 | 5251 |
| 20 | 2,3 | negation | 2 | 16/16 | 923 | 760 | 4102 |
| 24 | 2,3 | plain | 1 | 16/16 | 4398 | 0 | 18458 |
| 24 | 2,3 | negation | 2 | 16/16 | 3674 | 3306 | 15578 |
| 28 | 2,3 | plain | 1 | 16/16 | 16360 | 0 | 66330 |
| 28 | 2,3 | negation | 2 | 16/16 | 11260 | 10877 | 45975 |

### D = 0: generic (D=0, large |d_K|)

Structure (means over repetitions). `P(dec)` = share of trials with at least one decomposition; `ops/trial` = field multiplications + inversions of the decomposition sweeps per trial; `LA` = elimination multiply-adds summed over all attempts; `rule` = repetitions that met the 90% rule; `cert` = targets whose log passed `[k]G == T`.

| bits | m | arm | points | columns | P(dec) | trials/rel | ops/trial | relations | LA ops | LA rows x cols | rule | cert |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 12 | 2 | A3 | 47 | 12 | 0.94 | 0.446 | 330 | 17.7 | 100 | 17.7x12 | 20/20 | 16/16 |
| 12 | 2 | A0 | 47 | 47 | 0.92 | 0.468 | 330 | 72.7 | 555 | 72.7x47 | 20/20 | 16/16 |
| 12 | 3 | A3 | 16 | 4 | 0.95 | 0.428 | 1936 | 5.15 | 22.5 | 5.15x4 | 20/20 | 16/16 |
| 12 | 3 | A0 | 16 | 16 | 0.95 | 0.336 | 1936 | 25.5 | 221 | 25.5x16 | 20/20 | 16/16 |
| 16 | 2 | A3 | 204 | 51 | 0.87 | 0.508 | 1429 | 80.2 | 640 | 80.2x51 | 20/20 | 16/16 |
| 16 | 2 | A0 | 204 | 204 | 0.87 | 0.494 | 1429 | 307 | 2610 | 307x204 | 20/20 | 16/16 |
| 16 | 3 | A3 | 36 | 9 | 0.84 | 0.641 | 9396 | 11.9 | 93.9 | 11.9x9 | 20/20 | 16/16 |
| 16 | 3 | A0 | 36 | 36 | 0.78 | 0.628 | 9396 | 50 | 567 | 50x36 | 20/20 | 16/16 |
| 20 | 2 | A3 | 744 | 186 | 0.86 | 0.498 | 5209 | 280 | 2405 | 280x186 | 10/10 | 16/16 |
| 20 | 2 | A0 | 744 | 744 | 0.86 | 0.501 | 5209 | 1117 | 10496 | 1117x744 | 10/10 | 16/16 |
| 20 | 3 | A3 | 84 | 21 | 0.80 | 0.674 | 50148 | 21.8 | 150 | 21.8x21 | 10/10 | 16/16 |
| 20 | 3 | A0 | 84 | 84 | 0.74 | 0.748 | 50148 | 93.2 | 1481 | 93.2x84 | 10/10 | 16/16 |
| 24 | 2 | A3 | 3475 | 869 | 0.76 | 0.497 | 24326 | 1305 | 12146 | 1305x869 | 6/6 | 16/16 |
| 24 | 2 | A0 | 3475 | 3475 | 0.86 | 0.503 | 24326 | 5214 | 49891 | 5214x3475 | 6/6 | 16/16 |
| 24 | 3 | A3 | 232 | 58 | 0.72 | 0.77 | 3.79e+05 | 59.3 | 695 | 59.3x58 | 6/6 | 16/16 |
| 24 | 3 | A0 | 232 | 232 | 0.74 | 0.759 | 3.79e+05 | 252 | 7315 | 252x232 | 6/6 | 16/16 |
| 28 | 2 | A3 | 12934 | 3234 | 0.87 | 0.494 | 90539 | 4853 | 46002 | 4853x3234 | 3/3 | 16/16 |
| 28 | 2 | A0 | 12934 | 12934 | 0.87 | 0.498 | 90539 | 19402 | 1.93e+05 | 19402x12934 | 3/3 | 16/16 |
| 28 | 3 | A3 | 552 | 138 | 0.71 | 0.761 | 2.14e+06 | 161 | 4761 | 161x138 | 3/3 | 16/16 |
| 28 | 3 | A0 | 552 | 552 | 0.74 | 0.76 | 2.14e+06 | 552 | 33674 | 552x552 | 3/3 | 16/16 |

Cost in GAE (priced group operations + w x native operations; relation phase includes the base build; single-instance = relation phase + mean target stage; `rho` = best matched variant, mean per target, with its name; `IC/rho` = single-instance total over that rho). Mean +- standard error of the repetitions is in `small_de_derived.json`.

| bits | m | arm | rel w=0 | rel w=0.1 | rel w=1 | single w=0 | single w=0.1 | single w=1 | rho w=0 | rho w=0.1 | rho w=1 | IC/rho w=0 | w=0.1 | w=1 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 12 | 2 | A3 | 972 | 1358 | 4837 | 1200 | 1637 | 5569 | 131 (negation) | 138 (negation) | 139 (plain) | 9.19 | 11.9 | 40.2 |
| 12 | 2 | A0 | 332 | 1983 | 16841 | 563 | 2259 | 17528 | 131 (negation) | 138 (negation) | 139 (plain) | 4.31 | 16.4 | 126 |
| 12 | 3 | A3 | 543 | 1088 | 5991 | 816 | 1640 | 9054 | 129 (negation) | 136 (negation) | 143 (plain) | 6.32 | 12.1 | 63.5 |
| 12 | 3 | A0 | 513 | 2509 | 20476 | 783 | 3043 | 23382 | 129 (negation) | 136 (negation) | 143 (plain) | 6.06 | 22.4 | 164 |
| 16 | 2 | A3 | 4282 | 12044 | 81899 | 4990 | 12942 | 84511 | 242 (negation) | 259 (negation) | 280 (plain) | 20.6 | 49.9 | 302 |
| 16 | 2 | A0 | 1147 | 30206 | 2.92e+05 | 1858 | 31126 | 2.95e+05 | 242 (negation) | 259 (negation) | 280 (plain) | 7.67 | 120 | 1051 |
| 16 | 3 | A3 | 1782 | 10817 | 92126 | 2614 | 13608 | 1.13e+05 | 242 (negation) | 248 (plain) | 248 (plain) | 10.8 | 54.9 | 454 |
| 16 | 3 | A0 | 2828 | 37480 | 3.49e+05 | 3645 | 39949 | 3.67e+05 | 242 (negation) | 248 (plain) | 248 (plain) | 15 | 161 | 1479 |
| 20 | 2 | A3 | 20199 | 1.17e+05 | 9.87e+05 | 20635 | 1.18e+05 | 9.93e+05 | 824 (negation) | 889 (negation) | 1247 (plain) | 25.1 | 133 | 796 |
| 20 | 2 | A0 | 2115 | 3.9e+05 | 3.88e+06 | 2554 | 3.91e+05 | 3.89e+06 | 824 (negation) | 889 (negation) | 1247 (plain) | 3.1 | 440 | 3118 |
| 20 | 3 | A3 | 5122 | 1e+05 | 9.56e+05 | 5800 | 1.1e+05 | 1.04e+06 | 1018 (negation) | 1103 (negation) | 1404 (plain) | 5.7 | 99.4 | 744 |
| 20 | 3 | A0 | 12283 | 4.62e+05 | 4.51e+06 | 12914 | 4.7e+05 | 4.58e+06 | 1018 (negation) | 1103 (negation) | 1404 (plain) | 12.7 | 426 | 3263 |
| 24 | 2 | A3 | 1.04e+05 | 2.15e+06 | 2.06e+07 | 1.05e+05 | 2.16e+06 | 2.06e+07 | 2150 (negation) | 2328 (negation) | 3628 (plain) | 48.8 | 927 | 5691 |
| 24 | 2 | A0 | 8904 | 8.3e+06 | 8.29e+07 | 10007 | 8.31e+06 | 8.3e+07 | 2150 (negation) | 2328 (negation) | 3628 (plain) | 4.66 | 3568 | 22873 |
| 24 | 3 | A3 | 28164 | 2.25e+06 | 2.23e+07 | 29896 | 2.32e+06 | 2.29e+07 | 2310 (negation) | 2504 (negation) | 3663 (plain) | 12.9 | 925 | 6249 |
| 24 | 3 | A0 | 89598 | 9.34e+06 | 9.26e+07 | 91273 | 9.4e+06 | 9.32e+07 | 2310 (negation) | 2504 (negation) | 3663 (plain) | 39.5 | 3754 | 25437 |
| 28 | 2 | A3 | 5.25e+05 | 2.85e+07 | 2.81e+08 | 5.26e+05 | 2.86e+07 | 2.81e+08 | 13055 (negation) | 14322 (negation) | 19190 (plain) | 40.3 | 1994 | 14632 |
| 28 | 2 | A0 | 29700 | 1.13e+08 | 1.13e+09 | 30352 | 1.13e+08 | 1.13e+09 | 13055 (negation) | 14322 (negation) | 19190 (plain) | 2.32 | 7872 | 58738 |
| 28 | 3 | A3 | 1.57e+05 | 3.43e+07 | 3.42e+08 | 1.59e+05 | 3.46e+07 | 3.45e+08 | 12187 (negation) | 13368 (negation) | 15329 (plain) | 13 | 2591 | 22498 |
| 28 | 3 | A0 | 4.65e+05 | 1.16e+08 | 1.15e+09 | 4.67e+05 | 1.16e+08 | 1.16e+09 | 12187 (negation) | 13368 (negation) | 15329 (plain) | 38.3 | 8692 | 75523 |

Matched rho by variant (mean GAE per target; native = canonicalisations the unit does not charge; field ops = multiplications + inversions):

| bits | m | variant | A | runs | GAE | native | field ops |
|---|---|---|---|---|---|---|---|
| 12 | 2,3 | plain | 1 | 16/16 | 139 | 0 | 706 |
| 12 | 2,3 | negation | 2 | 16/16 | 131 | 69.1 | 678 |
| 16 | 2,3 | plain | 1 | 16/16 | 280 | 0 | 1298 |
| 16 | 2,3 | negation | 2 | 16/16 | 242 | 169 | 1157 |
| 20 | 2,3 | plain | 1 | 16/16 | 1247 | 0 | 5377 |
| 20 | 2,3 | negation | 2 | 16/16 | 824 | 659 | 3693 |
| 24 | 2,3 | plain | 1 | 16/16 | 3628 | 0 | 15361 |
| 24 | 2,3 | negation | 2 | 16/16 | 2150 | 1784 | 9455 |
| 28 | 2,3 | plain | 1 | 16/16 | 19190 | 0 | 77654 |
| 28 | 2,3 | negation | 2 | 16/16 | 13055 | 12676 | 53172 |

### Verdict against the frozen decision rule (A2 vs A1, relation phase)

`win` = A2 below A1 by more than 2 pooled standard errors, `tie` = within, `loss` = above. One entry per weight (0 / 0.1 / 1). A D is a gain only if every cell is `win` at all three weights, every target certified, and the 90% rule met in all repetitions.

| D | bits | m | rel-phase (w=0 / 0.1 / 1) | single-instance | A2/A1 rel ratio (w=0.1) | columns A2/A1 | LA A2/A1 | rule A2,A1 | cert A2,A1 |
|---|---|---|---|---|---|---|---|---|---|
| -3 | 12 | 2 | win / win / win | win / win / win | 0.506 | 16/48 | 0.375 | 20/20,20/20 | 16/16,16/16 |
| -3 | 12 | 3 | win / win / win | win / win / win | 0.368 | 5/15 | 0.140 | 20/20,20/20 | 16/16,16/16 |
| -3 | 16 | 2 | win / win / win | win / win / win | 0.372 | 68/204 | 0.337 | 20/20,20/20 | 16/16,16/16 |
| -3 | 16 | 3 | win / win / win | win / win / win | 0.408 | 12/36 | 0.323 | 20/20,20/20 | 16/16,16/16 |
| -3 | 20 | 2 | win / win / win | win / win / win | 0.341 | 247/741 | 0.301 | 10/10,10/10 | 16/16,16/16 |
| -3 | 20 | 3 | win / win / win | win / win / win | 0.327 | 28/84 | 0.211 | 10/10,10/10 | 16/16,16/16 |
| -3 | 24 | 2 | win / win / win | win / win / win | 0.332 | 1160/3480 | 0.320 | 6/6,6/6 | 16/16,16/16 |
| -3 | 24 | 3 | win / win / win | win / win / win | 0.347 | 77/231 | 0.176 | 6/6,6/6 | 16/16,16/16 |
| -3 | 28 | 2 | win / win / win | win / win / win | 0.336 | 4311/12933 | 0.315 | 3/3,3/3 | 16/16,16/16 |
| -3 | 28 | 3 | win / win / win | win / win / win | 0.337 | 184/552 | 0.152 | 3/3,3/3 | 16/16,16/16 |
| -4 | 12 | 2 | win / win / win | win / win / win | 0.583 | 23/46 | 0.504 | 20/20,20/20 | 16/16,16/16 |
| -4 | 12 | 3 | win / win / win | win / win / win | 0.594 | 7/14 | 0.346 | 20/20,20/20 | 16/16,16/16 |
| -4 | 16 | 2 | win / win / win | win / win / win | 0.515 | 102/204 | 0.468 | 20/20,20/20 | 16/16,16/16 |
| -4 | 16 | 3 | win / win / win | win / win / win | 0.518 | 18/36 | 0.451 | 20/20,20/20 | 16/16,16/16 |
| -4 | 20 | 2 | win / win / win | win / win / win | 0.507 | 371/742 | 0.469 | 10/10,10/10 | 16/16,16/16 |
| -4 | 20 | 3 | win / win / win | win / win / win | 0.555 | 41/82 | 0.393 | 10/10,10/10 | 16/16,16/16 |
| -4 | 24 | 2 | win / win / win | win / win / win | 0.503 | 1738/3476 | 0.470 | 6/6,6/6 | 16/16,16/16 |
| -4 | 24 | 3 | win / win / win | win / win / win | 0.536 | 115/230 | 0.487 | 6/6,6/6 | 16/16,16/16 |
| -4 | 28 | 2 | win / win / win | win / win / win | 0.498 | 6468/12936 | 0.496 | 3/3,3/3 | 16/16,16/16 |
| -4 | 28 | 3 | win / win / win | win / win / win | 0.473 | 276/552 | 0.302 | 3/3,3/3 | 16/16,16/16 |
| -7 | 12 | 2 | win / win / win | win / win / win | 0.445 | 12/43 | 0.229 | 20/20,20/20 | 16/16,16/16 |
| -7 | 12 | 3 | win / win / win | tie / tie / tie | 0.565 | 4/16 | 0.264 | 20/20,20/20 | 16/16,16/16 |
| -7 | 16 | 2 | win / win / win | win / win / win | 0.291 | 51/203 | 0.249 | 20/20,20/20 | 16/16,16/16 |
| -7 | 16 | 3 | win / win / win | win / win / win | 0.308 | 9/36 | 0.216 | 20/20,20/20 | 16/16,16/16 |
| -7 | 20 | 2 | win / win / win | win / win / win | 0.260 | 186/744 | 0.252 | 10/10,10/10 | 16/16,16/16 |
| -7 | 20 | 3 | win / win / win | win / win / win | 0.294 | 21/84 | 0.244 | 10/10,10/10 | 16/16,16/16 |
| -7 | 24 | 2 | win / win / win | win / win / win | 0.247 | 869/3476 | 0.247 | 6/6,6/6 | 16/16,16/16 |
| -7 | 24 | 3 | win / win / win | win / win / win | 0.298 | 58/232 | 0.187 | 6/6,6/6 | 16/16,16/16 |
| -7 | 28 | 2 | win / win / win | win / win / win | 0.246 | 3234/12936 | 0.236 | 3/3,3/3 | 16/16,16/16 |
| -7 | 28 | 3 | win / win / win | win / win / win | 0.287 | 138/552 | 0.173 | 3/3,3/3 | 16/16,16/16 |
| -8 | 12 | 2 | win / win / win | win / win / win | 0.457 | 12/47 | 0.162 | 20/20,20/20 | 16/16,16/16 |
| -8 | 12 | 3 | win / win / win | win / win / win | 0.501 | 4/16 | 0.202 | 20/20,20/20 | 16/16,16/16 |
| -8 | 16 | 2 | win / win / win | win / win / win | 0.289 | 51/203 | 0.223 | 20/20,20/20 | 16/16,16/16 |
| -8 | 16 | 3 | win / win / win | win / win / win | 0.333 | 9/36 | 0.161 | 20/20,20/20 | 16/16,16/16 |
| -8 | 20 | 2 | win / win / win | win / win / win | 0.260 | 186/742 | 0.236 | 10/10,10/10 | 16/16,16/16 |
| -8 | 20 | 3 | win / win / win | win / win / win | 0.282 | 21/84 | 0.244 | 10/10,10/10 | 16/16,16/16 |
| -8 | 24 | 2 | win / win / win | win / win / win | 0.248 | 869/3474 | 0.239 | 6/6,6/6 | 16/16,16/16 |
| -8 | 24 | 3 | win / win / win | win / win / win | 0.247 | 58/232 | 0.093 | 6/6,6/6 | 16/16,16/16 |
| -8 | 28 | 2 | win / win / win | win / win / win | 0.259 | 3234/12936 | 0.233 | 3/3,3/3 | 16/16,16/16 |
| -8 | 28 | 3 | win / win / win | win / win / win | 0.393 | 138/552 | 0.371 | 3/3,3/3 | 16/16,16/16 |
| -11 | 12 | 2 | loss / win / win | loss / win / win | 0.325 | 12/48 | 0.140 | 20/20,20/20 | 16/16,16/16 |
| -11 | 12 | 3 | win / win / win | win / win / win | 0.325 | 4/16 | 0.224 | 20/20,20/20 | 16/16,16/16 |
| -11 | 16 | 2 | loss / win / win | loss / win / win | 0.276 | 51/204 | 0.242 | 20/20,20/20 | 16/16,16/16 |
| -11 | 16 | 3 | win / win / win | win / win / win | 0.250 | 9/36 | 0.135 | 20/20,20/20 | 16/16,16/16 |
| -11 | 20 | 2 | loss / win / win | loss / win / win | 0.253 | 186/744 | 0.232 | 10/10,10/10 | 16/16,16/16 |
| -11 | 20 | 3 | win / win / win | win / win / win | 0.242 | 21/84 | 0.136 | 10/10,10/10 | 16/16,16/16 |
| -11 | 24 | 2 | loss / win / win | loss / win / win | 0.249 | 869/3473 | 0.234 | 6/6,6/6 | 16/16,16/16 |
| -11 | 24 | 3 | win / win / win | win / win / win | 0.249 | 58/232 | 0.126 | 6/6,6/6 | 16/16,16/16 |
| -11 | 28 | 2 | loss / win / win | loss / win / win | 0.247 | 3234/12935 | 0.234 | 3/3,3/3 | 16/16,16/16 |
| -11 | 28 | 3 | win / win / win | win / win / win | 0.262 | 138/552 | 0.080 | 3/3,3/3 | 16/16,16/16 |

Per D: D=-3: gain; D=-4: gain; D=-7: gain; D=-8: gain; D=-11: not a gain in every cell

### Control: folding without a small D_E (A3 vs A0 on the generic curve)

| bits | m | A3/A0 rel (w=0.1) | A3 vs A0 | columns A3/A0 | LA A3/A0 |
|---|---|---|---|---|---|
| 12 | 2 | 0.685 | loss/win/win | 12/47 | 0.181 |
| 12 | 3 | 0.434 | tie/win/win | 4/16 | 0.102 |
| 16 | 2 | 0.399 | loss/win/win | 51/204 | 0.245 |
| 16 | 3 | 0.289 | win/win/win | 9/36 | 0.166 |
| 20 | 2 | 0.300 | loss/win/win | 186/744 | 0.229 |
| 20 | 3 | 0.217 | win/win/win | 21/84 | 0.101 |
| 24 | 2 | 0.259 | loss/win/win | 869/3475 | 0.243 |
| 24 | 3 | 0.241 | win/win/win | 58/232 | 0.095 |
| 28 | 2 | 0.253 | loss/win/win | 3234/12934 | 0.238 |
| 28 | 3 | 0.296 | win/win/win | 138/552 | 0.141 |

A2/A1 minus A3/A0 for the layered D (relation phase, w=0.1; a negative number means the endomorphism arm is cheaper than the generic scalar-folded arm):

| D | bits | m | A2/A1 | A3/A0 | difference |
|---|---|---|---|---|---|
| -7 | 12 | 2 | 0.445 | 0.685 | -0.240 |
| -7 | 12 | 3 | 0.565 | 0.434 | +0.131 |
| -7 | 16 | 2 | 0.291 | 0.399 | -0.108 |
| -7 | 16 | 3 | 0.308 | 0.289 | +0.019 |
| -7 | 20 | 2 | 0.260 | 0.300 | -0.040 |
| -7 | 20 | 3 | 0.294 | 0.217 | +0.077 |
| -7 | 24 | 2 | 0.247 | 0.259 | -0.012 |
| -7 | 24 | 3 | 0.298 | 0.241 | +0.057 |
| -7 | 28 | 2 | 0.246 | 0.253 | -0.008 |
| -7 | 28 | 3 | 0.287 | 0.296 | -0.009 |
| -8 | 12 | 2 | 0.457 | 0.685 | -0.228 |
| -8 | 12 | 3 | 0.501 | 0.434 | +0.068 |
| -8 | 16 | 2 | 0.289 | 0.399 | -0.110 |
| -8 | 16 | 3 | 0.333 | 0.289 | +0.044 |
| -8 | 20 | 2 | 0.260 | 0.300 | -0.040 |
| -8 | 20 | 3 | 0.282 | 0.217 | +0.066 |
| -8 | 24 | 2 | 0.248 | 0.259 | -0.011 |
| -8 | 24 | 3 | 0.247 | 0.241 | +0.005 |
| -8 | 28 | 2 | 0.259 | 0.253 | +0.006 |
| -8 | 28 | 3 | 0.393 | 0.296 | +0.097 |
| -11 | 12 | 2 | 0.325 | 0.685 | -0.360 |
| -11 | 12 | 3 | 0.325 | 0.434 | -0.108 |
| -11 | 16 | 2 | 0.276 | 0.399 | -0.123 |
| -11 | 16 | 3 | 0.250 | 0.289 | -0.039 |
| -11 | 20 | 2 | 0.253 | 0.300 | -0.046 |
| -11 | 20 | 3 | 0.242 | 0.217 | +0.026 |
| -11 | 24 | 2 | 0.249 | 0.259 | -0.010 |
| -11 | 24 | 3 | 0.249 | 0.241 | +0.008 |
| -11 | 28 | 2 | 0.247 | 0.253 | -0.006 |
| -11 | 28 | 3 | 0.262 | 0.296 | -0.034 |

### Prediction scorecard

#### Prediction 1 (D = -3, -4): columns /|Aut|/2, LA about its square, P(dec) and solving unchanged, gap to the automorphism rho at least the plain gap

| D | bits | m | cols A1/A2 (pred) | LA A1/A2 (pred = square) | P(dec) diff (SE) | ops/trial A2/A1 | gap A2/rho_aut >= A1/rho_plain at w=0,0.1,1 (values) |
|---|---|---|---|---|---|---|---|
| -3 | 12 | 2 | 3.00 (3) | 2.7 (9) | 0.018 (0.024) | 1.000 | False (7.6>=9.5, 12.4>=25.3, 42.3>=167.3) |
| -3 | 12 | 3 | 3.00 (3) | 7.1 (9) | 0.113 (0.042) | 1.000 | False (12.4>=13.5, 22.3>=41.4, 96.6>=292.4) |
| -3 | 16 | 2 | 3.00 (3) | 3.0 (9) | 0.006 (0.011) | 1.000 | False (9.0>=7.7, 63.0>=121.1, 403.9>=1140.9) |
| -3 | 16 | 3 | 3.00 (3) | 3.1 (9) | 0.010 (0.033) | 1.000 | False (12.5>=12.1, 82.0>=124.5, 518.4>=1136.6) |
| -3 | 20 | 2 | 3.00 (3) | 3.3 (9) | 0.001 (0.011) | 1.000 | False (5.7>=4.8, 300.5>=538.7, 1984.3>=5343.5) |
| -3 | 20 | 3 | 3.00 (3) | 4.7 (9) | 0.011 (0.035) | 1.000 | False (15.6>=17.7, 411.2>=607.7, 2704.8>=5918.1) |
| -3 | 24 | 2 | 3.00 (3) | 3.1 (9) | 0.003 (0.005) | 1.000 | False (3.9>=4.0, 1985.4>=3220.9, 11685.4>=32173.0) |
| -3 | 24 | 3 | 3.00 (3) | 5.7 (9) | 0.011 (0.033) | 1.000 | False (22.9>=30.8, 2041.4>=3137.9, 11858.9>=31102.1) |
| -3 | 28 | 2 | 3.00 (3) | 3.2 (9) | 0.002 (0.003) | 1.000 | False (3.0>=3.0, 8428.6>=10603.0, 48325.4>=106003.4) |
| -3 | 28 | 3 | 3.00 (3) | 6.6 (9) | 0.009 (0.020) | 1.000 | False (37.5>=40.0, 8440.4>=9908.9, 48228.6>=98729.3) |
| -4 | 12 | 2 | 2.00 (2) | 2.0 (4) | 0.024 (0.020) | 1.000 | False (9.6>=8.8, 20.7>=28.5, 98.9>=205.6) |
| -4 | 12 | 3 | 2.00 (2) | 2.9 (4) | 0.033 (0.050) | 1.000 | False (11.2>=10.6, 25.8>=32.7, 128.4>=231.5) |
| -4 | 16 | 2 | 2.00 (2) | 2.1 (4) | 0.004 (0.009) | 1.000 | False (8.5>=5.3, 90.0>=98.1, 566.5>=932.6) |
| -4 | 16 | 3 | 2.00 (2) | 2.2 (4) | 0.047 (0.036) | 1.000 | False (12.9>=9.5, 106.0>=106.8, 643.4>=982.6) |
| -4 | 20 | 2 | 2.00 (2) | 2.1 (4) | 0.004 (0.009) | 1.000 | False (4.2>=4.4, 297.9>=493.5, 1847.8>=4895.8) |
| -4 | 20 | 3 | 2.00 (2) | 2.5 (4) | 0.069 (0.039) | 1.000 | False (17.1>=15.4, 483.8>=511.3, 3066.5>=4974.3) |
| -4 | 24 | 2 | 2.00 (2) | 2.1 (4) | 0.002 (0.004) | 1.000 | False (3.0>=3.3, 1878.1>=2689.8, 11224.2>=26867.8) |
| -4 | 24 | 3 | 2.00 (2) | 2.1 (4) | 0.003 (0.012) | 1.000 | False (21.7>=22.6, 1994.3>=2296.7, 11768.7>=22763.4) |
| -4 | 28 | 2 | 2.00 (2) | 2.0 (4) | 0.007 (0.006) | 1.000 | False (2.7>=2.3, 8183.4>=8322.5, 46247.4>=83203.6) |
| -4 | 28 | 3 | 2.00 (2) | 3.3 (4) | 0.017 (0.027) | 1.000 | False (37.0>=39.7, 8324.1>=9855.9, 46946.8>=98200.9) |

#### Prediction 2 (D = -7, -8, -11: A2 costs no less than A1)

Cells where A2 wins at all three weights: 25 of 30; cells where A2 loses at some weight: 5.

#### Prediction 3 (no cell ranks an IC arm below its matched rho across the band)

Arm-cells with single-instance total below the best matched rho at all three weights: 0 of 120; at some weight: 0; the smallest IC/rho ratio per weight (w=0, 0.1, 1): 2.32, 7.62, 40.16.

### Distance to the matched rho (single-instance IC total / best matched rho), minimum over cells per arm

| arm | w=0 | w=0.1 | w=1 | cell of the minimum at w=0.1 |
|---|---|---|---|---|
| A0 | 2.32 | 16.43 | 126.50 | D=0 b=12 m=2 |
| A1 | 2.42 | 19.33 | 137.88 | D=-11 b=12 m=2 |
| A2 | 2.72 | 7.62 | 40.16 | D=-11 b=12 m=2 |
| A3 | 5.70 | 11.91 | 40.19 | D=0 b=12 m=2 |

### Compact views

Amortised over the 16 targets (relation phase / 16 + mean target stage), w = 0.1, divided by the best matched rho per target:

| D | arm | m | 12 bits | 16 bits | 20 bits | 24 bits | 28 bits |
|---|---|---|---|---|---|---|---|
| -3 | A1 | 2 | 6.61 | 14.82 | 58.07 | 377.10 | 1571.29 |
| -3 | A1 | 3 | 11.10 | 22.80 | 94.17 | 393.56 | 1631.23 |
| -3 | A2 | 2 | 5.92 | 8.66 | 22.40 | 127.17 | 529.64 |
| -3 | A2 | 3 | 9.11 | 15.53 | 49.98 | 165.43 | 616.62 |
| -4 | A1 | 2 | 6.65 | 14.80 | 39.00 | 235.17 | 1029.51 |
| -4 | A1 | 3 | 9.10 | 24.49 | 68.24 | 252.49 | 1146.49 |
| -4 | A2 | 2 | 5.95 | 9.92 | 20.95 | 119.51 | 513.23 |
| -4 | A2 | 3 | 8.30 | 17.50 | 47.71 | 149.33 | 574.87 |
| -7 | A1 | 2 | 6.63 | 11.53 | 46.26 | 327.62 | 1009.89 |
| -7 | A1 | 3 | 9.65 | 17.29 | 56.60 | 251.58 | 1003.96 |
| -7 | A2 | 2 | 5.81 | 5.73 | 14.58 | 82.87 | 250.28 |
| -7 | A2 | 3 | 17.22 | 12.13 | 29.33 | 96.69 | 313.87 |
| -8 | A1 | 2 | 5.80 | 9.83 | 32.71 | 193.86 | 1170.00 |
| -8 | A1 | 3 | 7.78 | 17.36 | 42.75 | 204.13 | 1264.29 |
| -8 | A2 | 2 | 5.15 | 5.43 | 10.21 | 50.17 | 304.70 |
| -8 | A2 | 3 | 8.82 | 12.64 | 22.68 | 63.53 | 535.03 |
| -11 | A1 | 2 | 2.97 | 5.56 | 25.16 | 130.18 | 572.99 |
| -11 | A1 | 3 | 4.35 | 8.45 | 27.28 | 158.88 | 620.67 |
| -11 | A2 | 2 | 2.29 | 2.42 | 7.29 | 33.19 | 142.15 |
| -11 | A2 | 3 | 3.58 | 5.64 | 12.03 | 51.20 | 179.64 |
| 0 | A0 | 2 | 2.91 | 10.83 | 28.68 | 224.65 | 492.90 |
| 0 | A0 | 3 | 5.09 | 19.40 | 33.06 | 257.16 | 569.05 |
| 0 | A3 | 2 | 2.64 | 6.37 | 9.40 | 60.12 | 125.45 |
| 0 | A3 | 3 | 4.57 | 13.98 | 14.24 | 81.30 | 182.81 |

Secondary, labelled: field operations of the relation phase (multiplications + inversions + elimination multiply-adds + 4 per priced group operation; membership tests excluded) over the field operations of the best matched rho per target (its exact tally):

| D | arm | m | 12 bits | 16 bits | 20 bits | 24 bits | 28 bits |
|---|---|---|---|---|---|---|---|
| -3 | A1 | 2 | 30 | 215 | 1173 | 8298 | 35230 |
| -3 | A1 | 3 | 48 | 216 | 1546 | 7991 | 34913 |
| -3 | A2 | 2 | 14 | 78 | 399 | 2753 | 11831 |
| -3 | A2 | 3 | 15 | 87 | 505 | 2769 | 11753 |
| -4 | A1 | 2 | 39 | 240 | 1027 | 5972 | 27502 |
| -4 | A1 | 3 | 38 | 255 | 1238 | 5860 | 29117 |
| -4 | A2 | 2 | 21 | 122 | 520 | 3004 | 13685 |
| -4 | A2 | 3 | 22 | 131 | 686 | 3143 | 13772 |
| -7 | A1 | 2 | 37 | 215 | 1238 | 10299 | 33179 |
| -7 | A1 | 3 | 40 | 248 | 1204 | 7295 | 28729 |
| -7 | A2 | 2 | 14 | 60 | 319 | 2543 | 8147 |
| -7 | A2 | 3 | 21 | 75 | 353 | 2175 | 8247 |
| -8 | A1 | 2 | 26 | 183 | 905 | 5961 | 38202 |
| -8 | A1 | 3 | 30 | 228 | 936 | 5638 | 38919 |
| -8 | A2 | 2 | 10 | 51 | 233 | 1480 | 9885 |
| -8 | A2 | 3 | 14 | 74 | 264 | 1390 | 15301 |
| -11 | A1 | 2 | 27 | 121 | 728 | 4147 | 19113 |
| -11 | A1 | 3 | 26 | 130 | 588 | 4612 | 19636 |
| -11 | A2 | 2 | 7 | 32 | 183 | 1032 | 4713 |
| -11 | A2 | 3 | 8 | 32 | 142 | 1148 | 5151 |
| 0 | A0 | 2 | 22 | 202 | 827 | 6847 | 16502 |
| 0 | A0 | 3 | 26 | 244 | 793 | 7161 | 18095 |
| 0 | A3 | 2 | 10 | 68 | 227 | 1735 | 4141 |
| 0 | A3 | 3 | 10 | 67 | 170 | 1725 | 5362 |


## Limitations

- One curve per `(bits, D)`; the variance across curves with the same `D` is not
  measured, and the generic curve differs from the CM curves, so A0 and A1 are
  different groups. Repetitions vary only the trial walk and jumps
  `[pcc:unverified]`.
- The fold factor of the layered bases and the control is fixed at 4 (Amendment 1).
  The effect scales with the number of layers: nothing measured here says what
  happens at other values. By the same accounting a base folded by a scalar
  could be folded far deeper on any curve; as a derivation, not a measurement,
  the sweep work per relation then tends to a BSGS-like cost. Not run.
- The native work is priced only through the weights 0, 0.1 and 1; at w = 0 the
  decomposition sweeps are free and IC arms are compared on priced operations
  alone, at w = 1 every field multiplication costs a group addition. Nothing
  here is a calibrated wall-time claim. The secondary field-operation view
  (compact views, last table) is labelled and excludes membership tests
  `[pcc:unverified]`.
- `psi` evaluation is charged analytically (kernel size minus one group
  additions plus two multiplications per image), not by counted operations in
  the Velu code. The control's base build uses counted scalar multiplications
  (about 36 group operations per image), which is why A3 loses to A0 at w = 0.
- Membership in the adapted base is one hash probe on `x`, not on the invariant
  `u = x^3` or `u = x^2` (no computation of `u` is charged; this can only favour
  A2 by one or two multiplications per test).
- Targets are solved from the first repetition's known columns only; a target
  needs a decomposition into known points, so the descent depends on that
  repetition's determined set (at least 90% of the columns).
- Sizes are 12 to 28 bits (the 28-bit tier is an addition to the 12 to 24 bits
  of the task, within the protocol's "as large as the solver allows"); `r` is
  about `p/h` with `h` at most 4.
- The linear algebra is sparse modular elimination, counted as multiply-adds;
  no block Wiedemann or Lanczos is priced.
- Curve construction (one-time, a few milliseconds per curve up to 28 bits,
  logged in `run.log`) is excluded from every total, as the protocol says.

## Deviations from the task text

- Added 28 bits and the control arm A3, both pinned in Amendment 1 before data.
- Matched rho for `D = -7, -8, -11` and generic is negation (`A = 2`) and plain,
  as no automorphism group is usable; for `D = -3, -4` the group runs with
  `A = 6, 4` and its verification ledger is charged.
- No Groebner or Macaulay step for `m = 3` (nested quadratics instead); "largest
  matrix" is not applicable to the solving step.
- `benchmarks/progress/*` was regenerated by `benchrec report`, as the
  benchmark-record rule requires; it may conflict with other open pull requests
  that also regenerate it.

## Reproduce

```text
cargo build --release --bin small_de_ic
python3 tools/benchrec.py run $BENCHREC -- ./target/release/small_de_ic \
  --sizes 12,16,20,24,28 --reps 12:20,16:20,20:10,24:6,28:3 --targets 16 \
  --out-dir research/small_de_endo_fb_20261112
python3 research/small_de_endo_fb_20261112/render.py research/small_de_endo_fb_20261112/small_de.json
```

The run took 1 minute 44 seconds on the shared 4-core host (`[pcc:unverified]`).
Tests: `cargo test --release --lib chained_transfer::small_de` (7 pass:
`lambda_action_each_d`, `aut_base_is_closed_and_logs_transport`,
`layered_base_transports_logs_for_psi_and_scalar`,
`decompositions_verify_and_logs_certify`, `s3_quadratic_roots_are_the_sum_and_difference`,
`sparse_solver_matches_truth`, `generic_curve_has_large_dk`);
`cargo clippy --release --all-targets -- -D warnings` clean for the whole crate.
