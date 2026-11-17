# P3 (ECC2K-130 isogeny class): main-session reading of the frozen analyses

Masked analyses frozen at 6ac4230d59 before unmasking (PROTOCOL.md §10). Unmasked pooled files
hash-identical to the masked ones for all 13 families. Every card assessed `executed_as_specified`.
The frozen analyzer does not read `outcome`, so values vs bounds below are read from the rows by key
(director SE2). Isolation L0: times descriptive; Dreg, matrix shape and solution counts exact.

## Curves
n=131: E0 (b=1) + 262 level-263 curves (two Frobenius orbits A, B). n=43 analogue: E0 + 258 level-1
curves at the inert prime 257. Random-b controls (index >= 100000) where listed.

## C1: random factor-base subspace V (main arm)
| field | k | E0 Dreg | other class curves | outcome | rows differing from E0 in (Dreg, rows, cols) |
|---|---|---|---|---|---|
| n=131 | 3 | 6 | 6 (524/524) | value | 0 of 526 |
| n=131 | 4 | 7 | 7 (524/524) | value | 0 of 526 |
| n=131 | 5 | >=8 | >=8 (524/524) | bound (dmax 7) | n/a |
| n=43 | 3 | 7 | 7 (5160/5160) | value | 121 of 5180, all in one target (attempt 8); Dreg 7 everywhere, column count 433-481 varies with b, E0 in the modal shape (481 cols, 138 curves) |
| n=43 | 4 | 8 | 8 (516/516) | value | 0 of 518 |
| n=43 | 5 | >=8 | >=8 | bound (dmax 7) | n/a |
| n=43, m=2 | 3,4,5,8 | as class | identical | value | see a-m2 analyses |

## C2: presentation family at n=131 (stdv: standard V in 3 random moduli; badapt: modulus = minpoly(b); bsparse: 6 b-derived generators)
| k | stdv: E0 / others | badapt: all curves | bsparse | family minimum E0 / others |
|---|---|---|---|---|
| 3, 4, 5 | 6 / 6 | 6 | 6 (k=5) | 6 / 6 (262/262) |
| 6 | 6 / 7 (783 of 783 non-E0 rows; median 20.7 s vs 0.6 s) | 6 (528/528) | 6 or 7 (60/24) | **6 / 6 (262/262, both attempts)** |

Random-b controls under badapt (nbadapt) also give 6 at k=5,6.

**Reading.** With the standard subspace, E0 has Dreg 6 and every other class curve has 7 at k=6. With
the b-adapted presentation (modulus = minimal polynomial of b, so b -> z) every one of the 262 curves
reaches E0's Dreg 6, and so do random curves outside the class. The gap is the known "b = 1 is short in
the polynomial basis" presentation effect (cryptanalysis/experiments/ecc2k130-isogeny-class), not an
isogeny or volcano-level effect, and not a b-in-F2 structural effect (PROTOCOL C2' outcome N).
Isomorphic and A/A controls: identical counts in every checked cell (a-ctrl, b-ctrl analyses).
Planted instances: mostly `atleast` (planted solutions raise the closure degree past dmax); the planted
rows serve as the detector check (planted_ok), not as a Dreg comparison.

Not run: P3a-Y (n=43 k=13 relation yield), deferred under A1.
