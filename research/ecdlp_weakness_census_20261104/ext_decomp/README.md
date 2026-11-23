# ext_decomp: exact decomposition proportion on small extension fields

Node `HEUR-BK-GG-DECOMP-PROPORTION`: fraction of R in E(F_{q^n}) that is a sum of n points of the factor base F = {P : x(P) in F_q}.

Command (single thread, about 14 s): `cargo run --release`. Output: `results/ext_decomp.json`, `results/ext_decomp.csv` (both carry a `curve_over_subfield` flag), `results/table_primary.md`, `results/table_control.md`. Seed 20261104 (splitmix64).

Conventions: F_{q^n} = F_q[t]/(f), f the first monic irreducible found by seeded search (checked by t^{q^n}=t and gcd(t^{q^{n/p}}-t, f)=1). Curves y^2=x^3+ax+b. Primary rows (`curve_over_subfield: false`): a, b drawn uniformly from F_{q^n}, rejecting both-in-F_q and 4a^3+27b^2=0, the first two accepted per field. Control rows (`true`): a, b in F_q, the two of 8 seeded candidates with the largest (largest prime factor of #E)/#E. In the tables a and b are shown as integer encodings of the coefficient vector (sum c_i q^i). #E is counted over all x in F_{q^n}; square roots by Tonelli-Shanks (each root squared back and compared). Both y of each x in F_q are in F. M(R) counts unordered multisets of size n from F (repetition allowed) summing to R; the ordered count N(R) is also computed for n=2,3 and has the same support as M in every row (both families). "decomposable" = #{R : M(R)>=1}/#E; ratio = decomposable/(1/n!). The full histogram of M is in the JSON/CSV.

## Primary table: a, b in F_{q^n}

| q | n | curve (a_enc,b_enc) | #E | factorisation | \|F\| | \|F\|/q | decomposable | 1/n! | 1-exp(-1/n!) | ratio |
|---|---|---|---|---|---|---|---|---|---|---|
| 101 | 2 | (3589,7613) | 10281 | 3*23*149 | 104 | 1.0297 | 0.41329 | 0.50000 | 0.39347 | 0.8266 |
| 101 | 2 | (9773,6301) | 10084 | 2^2*2521 | 96 | 0.9505 | 0.36870 | 0.50000 | 0.39347 | 0.7374 |
| 211 | 2 | (43891,22176) | 44755 | 5*8951 | 200 | 0.9479 | 0.36128 | 0.50000 | 0.39347 | 0.7226 |
| 211 | 2 | (20532,14766) | 44732 | 2^2*53*211 | 196 | 0.9289 | 0.35042 | 0.50000 | 0.39347 | 0.7008 |
| 401 | 2 | (99106,138247) | 160278 | 2*3*26713 | 378 | 0.9426 | 0.36109 | 0.50000 | 0.39347 | 0.7222 |
| 401 | 2 | (33112,109792) | 161142 | 2*3*107*251 | 410 | 1.0224 | 0.40837 | 0.50000 | 0.39347 | 0.8167 |
| 23 | 3 | (9244,901) | 12179 | 19*641 | 22 | 0.9565 | 0.14139 | 0.16667 | 0.15352 | 0.8483 |
| 23 | 3 | (9046,11372) | 12096 | 2^6*3^3*7 | 20 | 0.8696 | 0.10103 | 0.16667 | 0.15352 | 0.6062 |
| 41 | 3 | (37854,50812) | 68564 | 2^2*61*281 | 40 | 0.9756 | 0.14500 | 0.16667 | 0.15352 | 0.8700 |
| 41 | 3 | (57651,65130) | 69018 | 2*3*11503 | 42 | 1.0244 | 0.16457 | 0.16667 | 0.15352 | 0.9874 |
| 61 | 3 | (94710,52627) | 227530 | 2*5*61*373 | 60 | 0.9836 | 0.14622 | 0.16667 | 0.15352 | 0.8773 |
| 61 | 3 | (112299,203239) | 227529 | 3^4*53^2 | 56 | 0.9180 | 0.12214 | 0.16667 | 0.15352 | 0.7328 |
| 11 | 4 | (12108,2158) | 14661 | 3^4*181 | 12 | 1.0909 | 0.06514 | 0.04167 | 0.04081 | 1.5633 |
| 11 | 4 | (14123,6196) | 14841 | 3^2*17*97 | 10 | 0.9091 | 0.03376 | 0.04167 | 0.04081 | 0.8102 |
| 13 | 4 | (3433,26310) | 28769 | 13*2213 | 10 | 0.7692 | 0.01741 | 0.04167 | 0.04081 | 0.4179 |
| 13 | 4 | (8965,18200) | 28594 | 2*17*29^2 | 18 | 1.3846 | 0.15671 | 0.04167 | 0.04081 | 3.7611 |

## Control table: a, b in F_q (curve defined over the subfield)

| q | n | curve (a_enc,b_enc) | #E | factorisation | \|F\| | \|F\|/q | decomposable | 1/n! | 1-exp(-1/n!) | ratio |
|---|---|---|---|---|---|---|---|---|---|---|
| 101 | 2 | (44,80) | 10283 | 7*13*113 | 202 | 2.0000 | 1.00000 | 0.50000 | 0.39347 | 2.0000 |
| 101 | 2 | (32,61) | 10304 | 2^6*7*23 | 199 | 1.9703 | 0.25000 | 0.50000 | 0.39347 | 0.5000 |
| 211 | 2 | (104,209) | 44655 | 3*5*13*229 | 422 | 2.0000 | 1.00000 | 0.50000 | 0.39347 | 2.0000 |
| 211 | 2 | (129,47) | 44719 | 197*227 | 422 | 2.0000 | 1.00000 | 0.50000 | 0.39347 | 2.0000 |
| 401 | 2 | (190,355) | 160448 | 2^6*23*109 | 801 | 1.9975 | 0.50000 | 0.50000 | 0.39347 | 1.0000 |
| 401 | 2 | (389,181) | 161504 | 2^5*7^2*103 | 799 | 1.9925 | 0.25000 | 0.50000 | 0.39347 | 0.5000 |
| 23 | 3 | (4,5) | 12380 | 2^2*5*619 | 19 | 0.8261 | 0.00162 | 0.16667 | 0.15352 | 0.0097 |
| 23 | 3 | (19,7) | 12366 | 2*3^3*229 | 17 | 0.7391 | 0.00146 | 0.16667 | 0.15352 | 0.0087 |
| 41 | 3 | (6,20) | 69444 | 2^2*3^3*643 | 35 | 0.8537 | 0.00052 | 0.16667 | 0.15352 | 0.0031 |
| 41 | 3 | (27,20) | 69444 | 2^2*3^3*643 | 35 | 0.8537 | 0.00052 | 0.16667 | 0.15352 | 0.0031 |
| 61 | 3 | (7,58) | 227450 | 2*5^2*4549 | 49 | 0.8033 | 0.00022 | 0.16667 | 0.15352 | 0.0013 |
| 61 | 3 | (38,20) | 226624 | 2^6*3541 | 63 | 1.0328 | 0.00028 | 0.16667 | 0.15352 | 0.0017 |
| 11 | 4 | (2,2) | 14715 | 3^3*5*109 | 22 | 2.0000 | 0.00917 | 0.04167 | 0.04081 | 0.2202 |
| 11 | 4 | (1,7) | 14715 | 3^3*5*109 | 22 | 2.0000 | 0.00917 | 0.04167 | 0.04081 | 0.2202 |
| 13 | 4 | (11,9) | 28416 | 2^8*3*37 | 23 | 1.7692 | 0.00169 | 0.04167 | 0.04081 | 0.0405 |
| 13 | 4 | (8,12) | 28416 | 2^8*3*37 | 25 | 1.9231 | 0.00338 | 0.04167 | 0.04081 | 0.0811 |

## What the numbers show

Primary rows, n=2: |F|/q is between 0.93 and 1.03, the decomposable fraction is between 0.350 and 0.413 (ratio to 1/2! between 0.70 and 0.83), and mean M(R) is between 0.43 and 0.53. [pcc:unverified]

Primary rows, n=3: |F|/q is between 0.87 and 1.02, the decomposable fraction is between 0.101 and 0.165 (ratio to 1/3! between 0.61 and 0.99), and mean M(R) is between 0.13 and 0.19. [pcc:unverified]

Primary rows, n=4: |F| is between 10 and 18 points and the decomposable fraction is between 0.0174 and 0.157 (ratio to 1/4! between 0.42 and 3.76), with mean M(R) between 0.025 and 0.21; the four rows are spread widely around the prediction. [pcc:unverified]

In the primary rows for n=2 and n=3, no decomposable fraction exceeds 1/n! (largest ratio 0.99); for n=2 all six rows lie between 0.35 and 0.41, close to 1 - exp(-1/2) = 0.393. [pcc:unverified]

Control rows (a, b in F_q): the decomposable fraction is 0.25 to 1.0 for n=2, 0.00027 to 0.00216 for n=3 and 0.0034 to 0.0099 for n=4, far from the primary rows for n=3 and n=4. Reading: for a curve defined over F_q the factor base is E(F_q) (n odd) or lies in a small subgroup, so the premise of the heuristic does not hold there. [pcc:unverified]

The ordered-tuple means for n=2 and n=3 equal |F|^n/#E in both families, and the ordered and multiset decomposable fractions coincide in every row. [pcc:unverified]
