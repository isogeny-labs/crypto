# Rank-deficit census (HEUR-RELATION-RANK, HEUR-PF-PKM-RANK-DELTA)

Command: `cargo run --release` from this directory (about 0.4 s, single thread, std only).
Outputs: `results/rank_deficit.csv`, `results/rank_deficit.json`, `results/tables.md`.

## Conventions

- Primes 1009, 2003, 4001, 8009, 16001 (all prime). Two curves y^2 = x^3 + ax + b per prime, prime order r, found by a seeded search (seed 0xC0FFEE xor p) and point counting by enumeration. G is the first point in (x, y) order.
- Factor base F = all points with x <= X, X chosen so that |F| = B is closest to sqrt(r). G is not forced into F. Unknowns are the B logs; "merged" uses B/2 columns, one per negation pair, with sign +1 / -1.
- Shape A: random aG decomposed as P1 + P2 with P1, P2 in F; row has +1 at P1 and P2 (2 if equal), rhs a. One row per draw (first P1 found); duplicate draws are kept. 5 samples per curve with seeds `0xA11CE ^ (p<<20) ^ (curve<<16) ^ (sample*7919+1)` (printed by the program and stored in the CSV). The first B+Delta rows of each sample are used, Delta in {0,1,2,3,5,10,20,40}.
- Shape B: every unordered pair {P1,P2} (P1 = P2 allowed) of F with P1 + P2 = P3 in F gives a row +1, +1, -1 with rhs 0.
- Rank is computed by Gaussian elimination over Z/r. The rank of [M | rhs] equals the rank of M in every run (asserted). deficit = B - rank, merged deficit = B/2 - merged rank.

Smallest Delta in {0,1,2,3,5,10,20,40} with deficit 0 in all 5 samples: for the full B columns no tested Delta reaches it on any of the 10 curves; for the merged columns see the last table. [pcc:unverified]

## Shape A

| p | curve | r | B | Delta | deficit min/med/max | merged deficit min/med/max |
|---|---|---|---|---|---|---|
| 1009 | 0 | 1039 | 32 | 0 | 4/6/7 | 0/1/1 |
| 1009 | 0 | 1039 | 32 | 1 | 4/5/7 | 0/1/1 |
| 1009 | 0 | 1039 | 32 | 2 | 4/5/7 | 0/1/1 |
| 1009 | 0 | 1039 | 32 | 3 | 4/5/7 | 0/1/1 |
| 1009 | 0 | 1039 | 32 | 5 | 3/5/5 | 0/0/1 |
| 1009 | 0 | 1039 | 32 | 10 | 2/3/5 | 0/0/0 |
| 1009 | 0 | 1039 | 32 | 20 | 0/1/2 | 0/0/0 |
| 1009 | 0 | 1039 | 32 | 40 | 0/0/1 | 0/0/0 |
| 1009 | 1 | 1033 | 32 | 0 | 4/6/6 | 0/0/1 |
| 1009 | 1 | 1033 | 32 | 1 | 4/6/6 | 0/0/1 |
| 1009 | 1 | 1033 | 32 | 2 | 4/5/6 | 0/0/1 |
| 1009 | 1 | 1033 | 32 | 3 | 3/4/6 | 0/0/1 |
| 1009 | 1 | 1033 | 32 | 5 | 3/4/6 | 0/0/1 |
| 1009 | 1 | 1033 | 32 | 10 | 2/2/4 | 0/0/1 |
| 1009 | 1 | 1033 | 32 | 20 | 0/2/3 | 0/0/0 |
| 1009 | 1 | 1033 | 32 | 40 | 0/1/2 | 0/0/0 |
| 2003 | 0 | 1979 | 44 | 0 | 6/7/8 | 0/0/1 |
| 2003 | 0 | 1979 | 44 | 1 | 5/6/8 | 0/0/1 |
| 2003 | 0 | 1979 | 44 | 2 | 4/6/8 | 0/0/1 |
| 2003 | 0 | 1979 | 44 | 3 | 4/5/7 | 0/0/1 |
| 2003 | 0 | 1979 | 44 | 5 | 4/5/7 | 0/0/1 |
| 2003 | 0 | 1979 | 44 | 10 | 4/4/5 | 0/0/0 |
| 2003 | 0 | 1979 | 44 | 20 | 2/2/3 | 0/0/0 |
| 2003 | 0 | 1979 | 44 | 40 | 0/1/2 | 0/0/0 |
| 2003 | 1 | 2087 | 46 | 0 | 5/6/8 | 0/0/2 |
| 2003 | 1 | 2087 | 46 | 1 | 4/5/8 | 0/0/2 |
| 2003 | 1 | 2087 | 46 | 2 | 4/5/7 | 0/0/1 |
| 2003 | 1 | 2087 | 46 | 3 | 4/5/7 | 0/0/1 |
| 2003 | 1 | 2087 | 46 | 5 | 3/5/7 | 0/0/1 |
| 2003 | 1 | 2087 | 46 | 10 | 2/5/7 | 0/0/1 |
| 2003 | 1 | 2087 | 46 | 20 | 1/3/5 | 0/0/1 |
| 2003 | 1 | 2087 | 46 | 40 | 0/1/2 | 0/0/0 |
| 4001 | 0 | 3917 | 62 | 0 | 7/11/13 | 0/1/2 |
| 4001 | 0 | 3917 | 62 | 1 | 7/11/13 | 0/1/2 |
| 4001 | 0 | 3917 | 62 | 2 | 6/11/12 | 0/1/2 |
| 4001 | 0 | 3917 | 62 | 3 | 5/11/12 | 0/1/2 |
| 4001 | 0 | 3917 | 62 | 5 | 4/9/10 | 0/1/2 |
| 4001 | 0 | 3917 | 62 | 10 | 2/7/8 | 0/0/2 |
| 4001 | 0 | 3917 | 62 | 20 | 2/5/6 | 0/0/2 |
| 4001 | 0 | 3917 | 62 | 40 | 1/2/5 | 0/0/2 |
| 4001 | 1 | 4007 | 64 | 0 | 6/9/12 | 0/1/2 |
| 4001 | 1 | 4007 | 64 | 1 | 6/9/12 | 0/1/2 |
| 4001 | 1 | 4007 | 64 | 2 | 5/8/12 | 0/1/2 |
| 4001 | 1 | 4007 | 64 | 3 | 5/7/12 | 0/1/2 |
| 4001 | 1 | 4007 | 64 | 5 | 4/6/11 | 0/0/2 |
| 4001 | 1 | 4007 | 64 | 10 | 3/6/11 | 0/0/2 |
| 4001 | 1 | 4007 | 64 | 20 | 1/6/8 | 0/0/1 |
| 4001 | 1 | 4007 | 64 | 40 | 0/1/5 | 0/0/1 |
| 8009 | 0 | 7867 | 88 | 0 | 10/12/20 | 0/0/1 |
| 8009 | 0 | 7867 | 88 | 1 | 10/12/19 | 0/0/1 |
| 8009 | 0 | 7867 | 88 | 2 | 10/12/19 | 0/0/1 |
| 8009 | 0 | 7867 | 88 | 3 | 10/12/19 | 0/0/1 |
| 8009 | 0 | 7867 | 88 | 5 | 9/11/19 | 0/0/1 |
| 8009 | 0 | 7867 | 88 | 10 | 8/11/17 | 0/0/1 |
| 8009 | 0 | 7867 | 88 | 20 | 4/8/16 | 0/0/0 |
| 8009 | 0 | 7867 | 88 | 40 | 1/5/11 | 0/0/0 |
| 8009 | 1 | 7933 | 90 | 0 | 12/14/16 | 0/1/3 |
| 8009 | 1 | 7933 | 90 | 1 | 12/14/16 | 0/1/3 |
| 8009 | 1 | 7933 | 90 | 2 | 12/14/15 | 0/1/3 |
| 8009 | 1 | 7933 | 90 | 3 | 12/14/15 | 0/1/3 |
| 8009 | 1 | 7933 | 90 | 5 | 11/14/15 | 0/1/3 |
| 8009 | 1 | 7933 | 90 | 10 | 9/12/13 | 0/1/3 |
| 8009 | 1 | 7933 | 90 | 20 | 8/9/12 | 0/1/1 |
| 8009 | 1 | 7933 | 90 | 40 | 3/4/5 | 0/0/0 |
| 16001 | 0 | 16097 | 126 | 0 | 18/21/24 | 0/0/3 |
| 16001 | 0 | 16097 | 126 | 1 | 18/20/23 | 0/0/3 |
| 16001 | 0 | 16097 | 126 | 2 | 17/20/23 | 0/0/3 |
| 16001 | 0 | 16097 | 126 | 3 | 17/20/22 | 0/0/2 |
| 16001 | 0 | 16097 | 126 | 5 | 15/19/21 | 0/0/2 |
| 16001 | 0 | 16097 | 126 | 10 | 13/16/21 | 0/0/2 |
| 16001 | 0 | 16097 | 126 | 20 | 7/14/15 | 0/0/1 |
| 16001 | 0 | 16097 | 126 | 40 | 4/9/12 | 0/0/1 |
| 16001 | 1 | 15887 | 126 | 0 | 19/25/28 | 0/2/2 |
| 16001 | 1 | 15887 | 126 | 1 | 19/24/27 | 0/2/2 |
| 16001 | 1 | 15887 | 126 | 2 | 19/23/27 | 0/2/2 |
| 16001 | 1 | 15887 | 126 | 3 | 18/23/27 | 0/2/2 |
| 16001 | 1 | 15887 | 126 | 5 | 18/23/26 | 0/2/2 |
| 16001 | 1 | 15887 | 126 | 10 | 16/21/23 | 0/1/2 |
| 16001 | 1 | 15887 | 126 | 20 | 10/17/20 | 0/1/2 |
| 16001 | 1 | 15887 | 126 | 40 | 7/13/15 | 0/0/1 |

## Shape B

| p | curve | r | B | free relations | rank | deficit | merged rank | merged deficit |
|---|---|---|---|---|---|---|---|---|
| 1009 | 0 | 1039 | 32 | 36 | 18 | 14 | 6 | 10 |
| 1009 | 1 | 1033 | 32 | 18 | 12 | 20 | 3 | 13 |
| 2003 | 0 | 1979 | 44 | 28 | 16 | 28 | 5 | 17 |
| 2003 | 1 | 2087 | 46 | 34 | 18 | 28 | 6 | 17 |
| 4001 | 0 | 3917 | 62 | 54 | 25 | 37 | 9 | 22 |
| 4001 | 1 | 4007 | 64 | 34 | 20 | 44 | 6 | 26 |
| 8009 | 0 | 7867 | 88 | 54 | 28 | 60 | 9 | 35 |
| 8009 | 1 | 7933 | 90 | 72 | 35 | 55 | 11 | 34 |
| 16001 | 0 | 16097 | 126 | 90 | 49 | 77 | 15 | 48 |
| 16001 | 1 | 15887 | 126 | 48 | 29 | 97 | 8 | 55 |

## Smallest Delta with deficit 0 in all 5 samples

| p | curve | r | full | merged |
|---|---|---|---|---|
| 1009 | 0 | 1039 | none<=40 | 10 |
| 1009 | 1 | 1033 | none<=40 | 20 |
| 2003 | 0 | 1979 | none<=40 | 10 |
| 2003 | 1 | 2087 | none<=40 | 40 |
| 4001 | 0 | 3917 | none<=40 | none<=40 |
| 4001 | 1 | 4007 | none<=40 | none<=40 |
| 8009 | 0 | 7867 | none<=40 | 20 |
| 8009 | 1 | 7933 | none<=40 | 40 |
| 16001 | 0 | 16097 | none<=40 | none<=40 |
| 16001 | 1 | 15887 | none<=40 | none<=40 |


## What the numbers show

In shape A on the B unknown columns, the deficit B - rank is nonzero in at least one sample at every tested Delta up to 40 on all 10 curves; at Delta = 40 the median deficit ranges from 0 (p = 1009, curve 0) to 13 (p = 16001, curve 1). [pcc:unverified]

At Delta = 10 the full-column deficit is between 2 and 21 (min over samples 2, max 23 for p = 16001 curve 1), so Delta = 10 gave full rank in none of the 50 samples. [pcc:unverified]

With negation pairs merged into B/2 columns, the deficit is smaller: it is 0 in all 5 samples at Delta = 10 for p = 1009 curve 0 and p = 2003 curve 0, at Delta = 20 for p = 1009 curve 1 and p = 8009 curve 0, at Delta = 40 for p = 2003 curve 1 and p = 8009 curve 1, and not within Delta <= 40 for p = 4001 (both curves) and p = 16001 (both curves). [pcc:unverified]

The deficit at a given Delta grows with p (B = 32 to 126): at Delta = 0 the median full deficit goes from 6 at p = 1009 to 21 and 25 at p = 16001. [pcc:unverified]

Shape B yields 18 to 90 free relations per curve, with rank 12 to 49 and deficit 14 to 97 on the B columns (merged deficit 10 to 55); the free relations alone are far from spanning the B columns on every curve. [pcc:unverified]
