### Table S1. Abscissa pairs that are exact Frobenius images, prefix window vs random 10-dimensional subspaces

Unordered pairs `{x0, x1}` in the large part of the window (`2^s <= x < 2^10`, nested random subspaces with the same dimensions) with `x1 = x0^(2^m)`, `1 <= m < n`; 8 seeded random subspaces per n. `E` is the random-window expectation `|W|^2 (n-1) / 2^(n+1)`.

| n | modulus | s | \|W\| | prefix window | random windows | E |
|---|---|---|---|---|---|---|
| 17 | `x^17+x^3+1` | 5 | 992 | 76 | 51, 53, 56, 57, 60, 63, 64, 79 (min 51, max 79) | 60.06 |
| 19 | `x^19+x^5+x^2+x+1` | 6 | 960 | 14 | 12, 13, 14, 16, 17, 17, 18, 19 (min 12, max 19) | 15.82 |
| 23 | `x^23+x^5+1` | 7 | 896 | 0 | 0, 0, 0, 1, 1, 2, 2, 3 (min 0, max 3) | 1.05 |
| 29 | `x^29+x^2+1` | 7 | 896 | 20 | 0, 0, 0, 0, 0, 0, 0, 0 (min 0, max 0) | 0.02 |
| 31 | `x^31+x^3+1` | 7 | 896 | 112 | 0, 0, 0, 0, 0, 0, 0, 0 (min 0, max 0) | 0.01 |

### Table S2. Mechanism spot checks (the bench's sparse modulus)

| n | modulus | alpha^(2^5) | alpha^n | note |
|---|---|---|---|---|
| 17 | `x^17+x^3+1` | `x^15+x^4+x` | `x^3+1` |  |
| 19 | `x^19+x^5+x^2+x+1` | `x^18+x^15+x^14+x^13` | `x^5+x^2+x+1` |  |
| 23 | `x^23+x^5+1` | `x^14+x^9` | `x^5+1` |  |
| 29 | `x^29+x^2+1` | `x^5+x^3` | `x^2+1` | n+1 = 32 = 2^5 would make the low-degree window nearly closed under pi^5 |
| 31 | `x^31+x^3+1` | `x^4+x` | `x^3+1` | n+1 = 32 = 2^5 would make the low-degree window nearly closed under pi^5 |

Spot checks at n = 31: `256^(2^26) = 258` (reviewer: 258), `300^(2^12) = 558` (reviewer: 558).

### Table S3. Merges of the measured large-point tables, split by cause (bench modulus)

`raw` = members that are exact Frobenius images of each other on the raw abscissae (`x1 = x0^(2^m)`); `projected only` = conjugate only after the cofactor projection. The recomputed `L`, orbits and merged points are compared with the seed-1 receipt of the merged run.

| n | L | orbits | merged | raw | projected only | multi-member orbits | largest orbit | E[pairs] random model | receipt (L, orbits, merged) | match |
|---|---|---|---|---|---|---|---|---|---|---|
| 17 | 474 | 410 | 64 | 41 | 23 | 61 | 3 | 54.7 | 474, 410, 64 | equal |
| 19 | 490 | 478 | 12 | 6 | 6 | 12 | 2 | 16.4 | 490, 478, 12 | equal |
| 23 | 444 | 442 | 2 | 0 | 2 | 2 | 2 | 1.0 | 444, 442, 2 | equal |
| 29 | 444 | 340 | 104 | 10 | 94 | 85 | 4 | 129.7 | 444, 340, 104 | equal |
| 31 | 460 | 426 | 34 | 30 | 4 | 25 | 8 | 4.4 | 460, 426, 34 | equal |

Largest orbit at n = 31 (raw abscissae of its members): [256, 258, 260, 262, 272, 274, 276, 278]

### Table S4. The same abstract curve over a seeded random (dense) modulus

Same group, `r` and cofactor; the prefix window `x < 2^10` is a different `F_2`-subspace of the field. `prefix pairs` as in S1.

| n | random modulus | terms | prefix pairs | L | orbits | merged | raw | projected only | E[pairs] random model |
|---|---|---|---|---|---|---|---|---|---|
| 17 | `x^17+x^16+x^13+x^11+x^10+x^7+x^5+x^4+x^3+x+1` | 11 | 50 | 495 | 446 | 49 | 22 | 27 | 59.7 |
| 19 | `x^19+x^18+x^16+x^15+x^14+x^13+x^11+x^6+x^2+x+1` | 11 | 12 | 494 | 483 | 11 | 7 | 4 | 16.7 |
| 23 | `x^23+x^22+x^19+x^18+x^15+x^14+x^9+x^8+x^7+x^6+x^4+x^2+1` | 13 | 0 | 453 | 453 | 0 | 0 | 0 | 1.1 |
| 29 | `x^29+x^28+x^25+x^20+x^17+x^16+x^15+x^12+x^10+x^8+x^6+x^3+1` | 13 | 0 | 433 | 330 | 103 | 0 | 103 | 123.4 |
| 31 | `x^31+x^27+x^26+x^24+x^22+x^21+x^20+x^18+x^14+x^12+x^11+x^4+x^2+x+1` | 15 | 0 | 448 | 446 | 2 | 0 | 2 | 4.2 |
