### Table B. Arm means over seeds

`complete` = rows emitted by the filter; `indep` = newly independent verified rows; `rank` out of S+1 columns; online CPU = collect+filter+replay, mean of the repeats; CPU per indep = mean over seeds with at least one independent row.

| series | n | cap | arm | seeds | reps | partials f/s/d | complete | indep | rank (of S+1) | online CPU ms | CPU ms per indep | complete per attempt |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| dose | 17 | 100 | dlp | 3 | 1 | 0.0/20.0/80.0 | 0.3 | 0.3 | 0.3 (15) | 0.89 | 1.052 (n=1) | 0.0033 |
| dose | 17 | 100 | dlp_orbit | 3 | 1 | 0.0/20.0/80.0 | 0.7 | 0.7 | 0.7 (15) | 0.84 | 0.847 (n=2) | 0.0067 |
| dose | 17 | 400 | dlp | 3 | 1 | 0.3/78.0/321.7 | 75.3 | 14.3 | 14.3 (15) | 3.60 | 0.252 (n=3) | 0.1883 |
| dose | 17 | 400 | dlp_orbit | 3 | 1 | 0.3/78.3/321.3 | 102.3 | 14.7 | 14.7 (15) | 3.87 | 0.264 (n=3) | 0.2558 |
| dose | 19 | 200 | dlp | 3 | 1 | 0.7/29.3/170.0 | 3.0 | 3.0 | 3.0 (32) | 1.90 | 0.692 (n=3) | 0.0150 |
| dose | 19 | 200 | dlp_orbit | 3 | 1 | 0.7/29.3/170.0 | 3.7 | 3.7 | 3.7 (32) | 1.78 | 0.584 (n=3) | 0.0183 |
| dose | 19 | 800 | dlp | 3 | 1 | 3.7/126.3/670.0 | 338.7 | 31.7 | 31.7 (32) | 9.90 | 0.313 (n=3) | 0.4233 |
| dose | 19 | 800 | dlp_orbit | 3 | 1 | 3.7/126.3/670.0 | 349.0 | 31.7 | 31.7 (32) | 10.61 | 0.335 (n=3) | 0.4362 |
| dose | 23 | 225 | dlp | 3 | 1 | 0.7/40.3/182.3 | 5.7 | 5.7 | 5.7 (54) | 3.57 | 0.955 (n=3) | 0.0252 |
| dose | 23 | 225 | dlp_orbit | 3 | 1 | 0.7/40.3/182.3 | 5.7 | 5.7 | 5.7 (54) | 3.82 | 1.027 (n=3) | 0.0252 |
| dose | 23 | 900 | dlp | 3 | 1 | 8.3/143.0/744.7 | 462.7 | 53.3 | 53.3 (54) | 20.50 | 0.384 (n=3) | 0.5141 |
| dose | 23 | 900 | dlp_orbit | 3 | 1 | 8.3/143.0/744.7 | 464.7 | 53.3 | 53.3 (54) | 20.31 | 0.381 (n=3) | 0.5163 |
| dose | 29 | 150 | dlp | 3 | 1 | 0.3/18.3/131.3 | 1.0 | 1.0 | 1.0 (62) | 258.17 | 258.165 (n=3) | 0.0067 |
| dose | 29 | 150 | dlp_orbit | 3 | 1 | 0.3/18.7/131.0 | 2.0 | 2.0 | 2.0 (62) | 278.67 | 206.206 (n=3) | 0.0133 |
| dose | 29 | 600 | dlp | 3 | 1 | 1.3/65.3/533.0 | 200.3 | 62.0 | 62.0 (62) | 1124.99 | 18.145 (n=3) | 0.3339 |
| dose | 29 | 600 | dlp_orbit | 3 | 1 | 1.3/66.0/532.3 | 285.3 | 62.0 | 62.0 (62) | 1102.30 | 17.779 (n=3) | 0.4756 |
| dose | 31 | 225 | dlp | 3 | 1 | 1.0/22.3/168.3 | 4.3 | 4.3 | 4.3 (66) | 4762.10 | 2240.156 (n=3) | 0.0193 |
| dose | 31 | 225 | dlp_orbit | 3 | 1 | 1.0/22.3/168.7 | 6.3 | 6.3 | 6.3 (66) | 4692.95 | 798.727 (n=3) | 0.0281 |
| dose | 31 | 900 | dlp | 3 | 1 | 3.3/80.7/687.0 | 334.7 | 66.0 | 66.0 (66) | 19145.12 | 290.078 (n=3) | 0.3719 |
| dose | 31 | 900 | dlp_orbit | 3 | 1 | 3.3/80.7/689.7 | 367.7 | 66.0 | 66.0 (66) | 18434.94 | 279.317 (n=3) | 0.4085 |
| primary | 17 | 200 | none | 5 | 1 | 6.6/0.0/0.0 | 6.6 | 6.6 | 6.6 (15) | 288.35 | 47.057 (n=5) | 0.0330 |
| primary | 17 | 200 | dlp | 5 | 3 | 0.4/37.8/161.8 | 6.4 | 6.2 | 6.2 (15) | 1.66 | 0.275 (n=5) | 0.0320 |
| primary | 17 | 200 | dlp_orbit | 5 | 3 | 0.4/37.8/161.8 | 10.8 | 9.2 | 9.2 (15) | 1.65 | 0.181 (n=5) | 0.0540 |
| primary | 17 | 200 | dlp_orbit_absorb | 5 | 1 | 1.8/48.6/149.6 | 15.6 | 11.0 | 11.0 (15) | 1.69 | 0.156 (n=5) | 0.0780 |
| primary | 19 | 400 | none | 5 | 1 | 28.2/0.0/0.0 | 28.2 | 27.2 | 27.2 (32) | 272.24 | 10.109 (n=5) | 0.0705 |
| primary | 19 | 400 | dlp | 5 | 3 | 2.4/65.4/332.2 | 49.8 | 25.6 | 25.6 (32) | 4.06 | 0.160 (n=5) | 0.1245 |
| primary | 19 | 400 | dlp_orbit | 5 | 3 | 2.4/65.4/332.2 | 53.2 | 26.2 | 26.2 (32) | 4.20 | 0.161 (n=5) | 0.1330 |
| primary | 19 | 400 | dlp_orbit_absorb | 5 | 1 | 5.8/89.8/304.4 | 65.4 | 29.0 | 29.0 (32) | 4.31 | 0.149 (n=5) | 0.1635 |
| primary | 23 | 450 | none | 5 | 1 | 9.6/0.0/0.0 | 9.6 | 9.6 | 9.6 (54) | 104.12 | 11.086 (n=5) | 0.0213 |
| primary | 23 | 450 | dlp | 5 | 3 | 3.6/86.8/357.4 | 92.6 | 50.8 | 50.8 (54) | 8.54 | 0.168 (n=5) | 0.2058 |
| primary | 23 | 450 | dlp_orbit | 5 | 3 | 3.6/87.0/357.2 | 94.2 | 50.8 | 50.8 (54) | 8.66 | 0.171 (n=5) | 0.2093 |
| primary | 23 | 450 | dlp_orbit_absorb | 5 | 1 | 5.4/100.2/343.8 | 102.0 | 50.8 | 50.8 (54) | 9.43 | 0.186 (n=5) | 0.2267 |
| primary | 29 | 300 | none | 5 | 1 | 6.0/0.0/0.0 | 6.0 | 6.0 | 6.0 (62) | 29000.13 | 5093.851 (n=5) | 0.0200 |
| primary | 29 | 300 | dlp | 5 | 3 | 1.2/34.8/263.6 | 19.4 | 19.2 | 19.2 (62) | 570.53 | 31.481 (n=5) | 0.0647 |
| primary | 29 | 300 | dlp_orbit | 5 | 3 | 1.2/35.0/263.4 | 51.8 | 51.6 | 51.6 (62) | 557.27 | 10.794 (n=5) | 0.1727 |
| primary | 29 | 300 | dlp_orbit_absorb | 5 | 1 | 4.0/55.2/240.8 | 68.2 | 58.4 | 58.4 (62) | 338.48 | 5.803 (n=5) | 0.2273 |
| primary | 31 | 450 | none | 5 | 1 | 3.2/0.0/0.0 | 3.2 | 3.2 | 3.2 (66) | 50851.31 | 18602.299 (n=5) | 0.0071 |
| primary | 31 | 450 | dlp | 5 | 2 | 1.8/38.4/339.4 | 45.6 | 45.6 | 45.6 (66) | 10247.61 | 226.632 (n=5) | 0.1013 |
| primary | 31 | 450 | dlp_orbit | 5 | 2 | 1.8/38.4/340.6 | 62.2 | 61.6 | 61.6 (66) | 10071.87 | 165.164 (n=5) | 0.1382 |
| primary | 31 | 450 | dlp_orbit_absorb | 5 | 1 | 6.8/75.2/360.8 | 110.0 | 65.6 | 65.6 (66) | 3288.19 | 50.110 (n=5) | 0.2444 |

### Table C. What the quotient did to the large columns

Random model: two distinct large points lie in one signed Frobenius orbit with probability about (2n-2)/(r-1), so E[colliding pairs] = C(L,2)(2n-2)/(r-1) for L large points. `merged` counts large points that share an orbit vertex with another large point (L minus orbits, absorbed excluded).

| n | curve | r | cofactor | L (large points) | orbits | merged | absorbed (absorb arm) | E[pairs] random model | L/r |
|---|---|---|---|---|---|---|---|---|---|
| 17 | K_1 | 65587 | 2 | 474 | 410 | 64 | 19 | 54.7 | 0.0072 |
| 19 | K_1 | 262543 | 2 | 490 | 478 | 12 | 18 | 16.4 | 0.0019 |
| 23 | K_1 | 4196903 | 2 | 444 | 442 | 2 | 9 | 1.0 | 0.0001 |
| 29 | K_1 | 42457 | 12646 | 444 | 340 | 104 | 38 | 129.7 | 0.0105 |
| 31 | K_0 | 1439393 | 1492 | 460 | 426 | 34 | 38 | 4.4 | 0.0003 |

### Table D. Paired per-seed differences against the plain double large prime arm (ii)

Counters are deterministic given the seed (the repeats agree exactly, see Integrity), so a nonzero counter difference is a property of the arm, not of timing. CPU differences are percent of the plain arm's online CPU.

| series | n | cap | treatment | seed | d complete | d indep | d rank | d online CPU % | d CPU per indep % |
|---|---|---|---|---|---|---|---|---|---|
| dose | 17 | 100 | dlp_orbit | 1 | +0 | +0 | +0 | +0.29 | n/a |
| dose | 17 | 100 | dlp_orbit | 2 | +0 | +0 | +0 | -16.39 | -16.39 |
| dose | 17 | 100 | dlp_orbit | 3 | +1 | +1 | +1 | +2.60 | n/a |
| dose | 17 | 400 | dlp_orbit | 1 | +24 | +0 | +0 | +3.11 | +3.11 |
| dose | 17 | 400 | dlp_orbit | 2 | +30 | +0 | +0 | +3.29 | +3.29 |
| dose | 17 | 400 | dlp_orbit | 3 | +27 | +1 | +1 | +16.78 | +8.44 |
| dose | 19 | 200 | dlp_orbit | 1 | +0 | +0 | +0 | -17.87 | -17.87 |
| dose | 19 | 200 | dlp_orbit | 2 | +0 | +0 | +0 | +4.53 | +4.53 |
| dose | 19 | 200 | dlp_orbit | 3 | +2 | +2 | +2 | -4.42 | -36.28 |
| dose | 19 | 800 | dlp_orbit | 1 | +11 | +0 | +0 | -2.07 | -2.07 |
| dose | 19 | 800 | dlp_orbit | 2 | +9 | +0 | +0 | -6.40 | -6.40 |
| dose | 19 | 800 | dlp_orbit | 3 | +11 | +0 | +0 | +33.35 | +33.35 |
| dose | 23 | 225 | dlp_orbit | 1 | +0 | +0 | +0 | +14.61 | +14.61 |
| dose | 23 | 225 | dlp_orbit | 2 | +0 | +0 | +0 | +8.21 | +8.21 |
| dose | 23 | 225 | dlp_orbit | 3 | +0 | +0 | +0 | -1.15 | -1.15 |
| dose | 23 | 900 | dlp_orbit | 1 | +2 | +0 | +0 | +8.24 | +8.24 |
| dose | 23 | 900 | dlp_orbit | 2 | +2 | +0 | +0 | -4.78 | -4.78 |
| dose | 23 | 900 | dlp_orbit | 3 | +2 | +0 | +0 | -5.25 | -5.25 |
| dose | 29 | 150 | dlp_orbit | 1 | +0 | +0 | +0 | +7.75 | +7.75 |
| dose | 29 | 150 | dlp_orbit | 2 | +0 | +0 | +0 | +3.27 | +3.27 |
| dose | 29 | 150 | dlp_orbit | 3 | +3 | +3 | +3 | +13.01 | -71.75 |
| dose | 29 | 600 | dlp_orbit | 1 | +90 | +0 | +0 | -3.01 | -3.01 |
| dose | 29 | 600 | dlp_orbit | 2 | +83 | +0 | +0 | +3.04 | +3.04 |
| dose | 29 | 600 | dlp_orbit | 3 | +82 | +0 | +0 | -6.20 | -6.20 |
| dose | 31 | 225 | dlp_orbit | 1 | +4 | +4 | +4 | -4.48 | -80.90 |
| dose | 31 | 225 | dlp_orbit | 2 | +0 | +0 | +0 | +1.53 | +1.53 |
| dose | 31 | 225 | dlp_orbit | 3 | +2 | +2 | +2 | -1.51 | -40.90 |
| dose | 31 | 900 | dlp_orbit | 1 | +36 | +0 | +0 | -5.41 | -5.41 |
| dose | 31 | 900 | dlp_orbit | 2 | +31 | +0 | +0 | -2.95 | -2.95 |
| dose | 31 | 900 | dlp_orbit | 3 | +32 | +0 | +0 | -2.55 | -2.55 |
| primary | 17 | 200 | dlp_orbit | 1 | +3 | +3 | +3 | -3.34 | -39.59 |
| primary | 17 | 200 | dlp_orbit | 2 | +6 | +4 | +4 | +4.77 | -41.79 |
| primary | 17 | 200 | dlp_orbit | 3 | +4 | +3 | +3 | -3.10 | -32.17 |
| primary | 17 | 200 | dlp_orbit | 4 | +5 | +3 | +3 | +5.52 | -26.14 |
| primary | 17 | 200 | dlp_orbit | 5 | +4 | +2 | +2 | -3.80 | -25.18 |
| primary | 17 | 200 | dlp_orbit_absorb | 1 | +6 | +4 | +4 | -0.17 | -44.54 |
| primary | 17 | 200 | dlp_orbit_absorb | 2 | +12 | +7 | +7 | -0.24 | -58.43 |
| primary | 17 | 200 | dlp_orbit_absorb | 3 | +7 | +6 | +6 | +4.74 | -43.60 |
| primary | 17 | 200 | dlp_orbit_absorb | 4 | +11 | +4 | +4 | +10.88 | -29.44 |
| primary | 17 | 200 | dlp_orbit_absorb | 5 | +10 | +3 | +3 | -4.03 | -32.82 |
| primary | 19 | 400 | dlp_orbit | 1 | +1 | +1 | +1 | +4.37 | +0.20 |
| primary | 19 | 400 | dlp_orbit | 2 | +1 | +0 | +0 | +0.06 | +0.06 |
| primary | 19 | 400 | dlp_orbit | 3 | +7 | +1 | +1 | -0.20 | -3.64 |
| primary | 19 | 400 | dlp_orbit | 4 | +3 | +1 | +1 | +7.17 | +2.71 |
| primary | 19 | 400 | dlp_orbit | 5 | +5 | +0 | +0 | +5.89 | +5.89 |
| primary | 19 | 400 | dlp_orbit_absorb | 1 | +14 | +6 | +6 | -0.17 | -20.13 |
| primary | 19 | 400 | dlp_orbit_absorb | 2 | +12 | +3 | +3 | +11.94 | -0.06 |
| primary | 19 | 400 | dlp_orbit_absorb | 3 | +21 | +2 | +2 | +3.61 | -3.30 |
| primary | 19 | 400 | dlp_orbit_absorb | 4 | +14 | +4 | +4 | +2.96 | -12.29 |
| primary | 19 | 400 | dlp_orbit_absorb | 5 | +17 | +2 | +2 | +12.23 | +4.75 |
| primary | 23 | 450 | dlp_orbit | 1 | +2 | +0 | +0 | +0.09 | +0.09 |
| primary | 23 | 450 | dlp_orbit | 2 | +2 | +0 | +0 | -0.23 | -0.23 |
| primary | 23 | 450 | dlp_orbit | 3 | +2 | +0 | +0 | -2.56 | -2.56 |
| primary | 23 | 450 | dlp_orbit | 4 | +2 | +0 | +0 | +10.72 | +10.72 |
| primary | 23 | 450 | dlp_orbit | 5 | +0 | +0 | +0 | -0.69 | -0.69 |
| primary | 23 | 450 | dlp_orbit_absorb | 1 | +9 | +0 | +0 | +20.04 | +20.04 |
| primary | 23 | 450 | dlp_orbit_absorb | 2 | +10 | +0 | +0 | +38.62 | +38.62 |
| primary | 23 | 450 | dlp_orbit_absorb | 3 | +10 | +0 | +0 | -6.59 | -6.59 |
| primary | 23 | 450 | dlp_orbit_absorb | 4 | +11 | +0 | +0 | -8.78 | -8.78 |
| primary | 23 | 450 | dlp_orbit_absorb | 5 | +7 | +0 | +0 | +3.72 | +3.72 |
| primary | 29 | 300 | dlp_orbit | 1 | +37 | +37 | +37 | -2.97 | -77.76 |
| primary | 29 | 300 | dlp_orbit | 2 | +37 | +37 | +37 | +2.38 | -65.27 |
| primary | 29 | 300 | dlp_orbit | 3 | +27 | +27 | +27 | -6.10 | -55.81 |
| primary | 29 | 300 | dlp_orbit | 4 | +34 | +34 | +34 | -0.69 | -62.08 |
| primary | 29 | 300 | dlp_orbit | 5 | +27 | +27 | +27 | -4.13 | -58.06 |
| primary | 29 | 300 | dlp_orbit_absorb | 1 | +61 | +48 | +48 | -36.56 | -88.17 |
| primary | 29 | 300 | dlp_orbit_absorb | 2 | +48 | +39 | +39 | -36.86 | -79.32 |
| primary | 29 | 300 | dlp_orbit_absorb | 3 | +42 | +35 | +35 | -45.24 | -77.72 |
| primary | 29 | 300 | dlp_orbit_absorb | 4 | +47 | +40 | +40 | -47.04 | -81.77 |
| primary | 29 | 300 | dlp_orbit_absorb | 5 | +46 | +34 | +34 | -35.80 | -75.49 |
| primary | 31 | 450 | dlp_orbit | 1 | +21 | +19 | +19 | -2.50 | -30.56 |
| primary | 31 | 450 | dlp_orbit | 2 | +18 | +18 | +18 | -0.95 | -28.38 |
| primary | 31 | 450 | dlp_orbit | 3 | +13 | +13 | +13 | -1.51 | -26.13 |
| primary | 31 | 450 | dlp_orbit | 4 | +14 | +14 | +14 | -1.75 | -25.07 |
| primary | 31 | 450 | dlp_orbit | 5 | +17 | +16 | +16 | -1.78 | -25.59 |
| primary | 31 | 450 | dlp_orbit_absorb | 1 | +57 | +19 | +19 | -64.84 | -74.96 |
| primary | 31 | 450 | dlp_orbit_absorb | 2 | +66 | +19 | +19 | -66.97 | -76.48 |
| primary | 31 | 450 | dlp_orbit_absorb | 3 | +70 | +26 | +26 | -68.39 | -81.03 |
| primary | 31 | 450 | dlp_orbit_absorb | 4 | +62 | +20 | +20 | -72.39 | -80.89 |
| primary | 31 | 450 | dlp_orbit_absorb | 5 | +67 | +16 | +16 | -66.96 | -74.97 |

### Table E. Aggregate of the paired differences

Mean +/- 95% Student-t half-width over S seeds; signs = in how many seeds the treatment exceeded / equalled / fell below plain. Timing control = mean |relative change| of online CPU between repeats of the same cell (plain and orbit arms pooled). A CPU difference is `within noise` if |mean| is at most its t half-width or at most the timing control, else `outside noise`. Treatment `dlp_orbit` is arm (iii); `dlp_orbit_absorb` is the secondary arm (iv).

| series | n | cap | treatment | S | d complete | signs | d indep | signs | d rank | d CPU per indep % | verdict (CPU per indep) | d online CPU % | verdict (online CPU) | timing control % |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| dose | 17 | 100 | dlp_orbit | 3 | +0.3 +/- 1.4 | 1+/20/0- | +0.3 +/- 1.4 | 1+/20/0- | +0.3 +/- 1.4 | -16.4 +/- n/a (n=1) | S=1 | -4.5 +/- 25.7 | within noise | n/a |
| dose | 17 | 400 | dlp_orbit | 3 | +27.0 +/- 7.5 | 3+/00/0- | +0.3 +/- 1.4 | 1+/20/0- | +0.3 +/- 1.4 | +4.9 +/- 7.5 (n=3) | within noise | +7.7 +/- 19.5 | within noise | n/a |
| dose | 19 | 200 | dlp_orbit | 3 | +0.7 +/- 2.9 | 1+/20/0- | +0.7 +/- 2.9 | 1+/20/0- | +0.7 +/- 2.9 | -16.5 +/- 50.8 (n=3) | within noise | -5.9 +/- 28.0 | within noise | n/a |
| dose | 19 | 800 | dlp_orbit | 3 | +10.3 +/- 2.9 | 3+/00/0- | +0.0 +/- 0.0 | 0+/30/0- | +0.0 +/- 0.0 | +8.3 +/- 54.2 (n=3) | within noise | +8.3 +/- 54.2 | within noise | n/a |
| dose | 23 | 225 | dlp_orbit | 3 | +0.0 +/- 0.0 | 0+/30/0- | +0.0 +/- 0.0 | 0+/30/0- | +0.0 +/- 0.0 | +7.2 +/- 19.7 (n=3) | within noise | +7.2 +/- 19.7 | within noise | n/a |
| dose | 23 | 900 | dlp_orbit | 3 | +2.0 +/- 0.0 | 3+/00/0- | +0.0 +/- 0.0 | 0+/30/0- | +0.0 +/- 0.0 | -0.6 +/- 19.0 (n=3) | within noise | -0.6 +/- 19.0 | within noise | n/a |
| dose | 29 | 150 | dlp_orbit | 3 | +1.0 +/- 4.3 | 1+/20/0- | +1.0 +/- 4.3 | 1+/20/0- | +1.0 +/- 4.3 | -20.2 +/- 111.0 (n=3) | within noise | +8.0 +/- 12.1 | within noise | n/a |
| dose | 29 | 600 | dlp_orbit | 3 | +85.0 +/- 10.8 | 3+/00/0- | +0.0 +/- 0.0 | 0+/30/0- | +0.0 +/- 0.0 | -2.1 +/- 11.7 (n=3) | within noise | -2.1 +/- 11.7 | within noise | n/a |
| dose | 31 | 225 | dlp_orbit | 3 | +2.0 +/- 5.0 | 2+/10/0- | +2.0 +/- 5.0 | 2+/10/0- | +2.0 +/- 5.0 | -40.1 +/- 102.4 (n=3) | within noise | -1.5 +/- 7.5 | within noise | n/a |
| dose | 31 | 900 | dlp_orbit | 3 | +33.0 +/- 6.6 | 3+/00/0- | +0.0 +/- 0.0 | 0+/30/0- | +0.0 +/- 0.0 | -3.6 +/- 3.8 (n=3) | within noise | -3.6 +/- 3.8 | within noise | n/a |
| primary | 17 | 200 | dlp_orbit | 5 | +4.4 +/- 1.4 | 5+/00/0- | +3.0 +/- 0.9 | 5+/00/0- | +3.0 +/- 0.9 | -33.0 +/- 9.4 (n=5) | outside noise | +0.0 +/- 5.8 | within noise | 8.0 |
| primary | 17 | 200 | dlp_orbit_absorb | 5 | +9.2 +/- 3.2 | 5+/00/0- | +4.8 +/- 2.0 | 5+/00/0- | +4.8 +/- 2.0 | -41.8 +/- 14.2 (n=5) | outside noise | +2.2 +/- 7.1 | within noise | 8.0 |
| primary | 19 | 400 | dlp_orbit | 5 | +3.4 +/- 3.2 | 5+/00/0- | +0.6 +/- 0.7 | 3+/20/0- | +0.6 +/- 0.7 | +1.0 +/- 4.4 (n=5) | within noise | +3.5 +/- 4.2 | within noise | 6.2 |
| primary | 19 | 400 | dlp_orbit_absorb | 5 | +15.6 +/- 4.4 | 5+/00/0- | +3.4 +/- 2.1 | 5+/00/0- | +3.4 +/- 2.1 | -6.2 +/- 12.4 (n=5) | within noise | +6.1 +/- 7.0 | within noise | 6.2 |
| primary | 23 | 450 | dlp_orbit | 5 | +1.6 +/- 1.1 | 4+/10/0- | +0.0 +/- 0.0 | 0+/50/0- | +0.0 +/- 0.0 | +1.5 +/- 6.5 (n=5) | within noise | +1.5 +/- 6.5 | within noise | 8.5 |
| primary | 23 | 450 | dlp_orbit_absorb | 5 | +9.4 +/- 1.9 | 5+/00/0- | +0.0 +/- 0.0 | 0+/50/0- | +0.0 +/- 0.0 | +9.4 +/- 24.7 (n=5) | within noise | +9.4 +/- 24.7 | within noise | 8.5 |
| primary | 29 | 300 | dlp_orbit | 5 | +32.4 +/- 6.3 | 5+/00/0- | +32.4 +/- 6.3 | 5+/00/0- | +32.4 +/- 6.3 | -63.8 +/- 10.7 (n=5) | outside noise | -2.3 +/- 4.1 | within noise | 4.0 |
| primary | 29 | 300 | dlp_orbit_absorb | 5 | +48.8 +/- 8.9 | 5+/00/0- | +39.2 +/- 6.9 | 5+/00/0- | +39.2 +/- 6.9 | -80.5 +/- 6.0 (n=5) | outside noise | -40.3 +/- 6.7 | outside noise | 4.0 |
| primary | 31 | 450 | dlp_orbit | 5 | +16.6 +/- 4.0 | 5+/00/0- | +16.0 +/- 3.2 | 5+/00/0- | +16.0 +/- 3.2 | -27.1 +/- 2.8 (n=5) | outside noise | -1.7 +/- 0.7 | within noise | 2.2 |
| primary | 31 | 450 | dlp_orbit_absorb | 5 | +64.4 +/- 6.2 | 5+/00/0- | +20.0 +/- 4.6 | 5+/00/0- | +20.0 +/- 4.6 | -77.7 +/- 3.8 (n=5) | outside noise | -67.9 +/- 3.5 | outside noise | 2.2 |

### Table F. No-large-prime arm (i) and absorb arm against the plain double large prime arm (ii), primary series

| n | cap | S | indep: none | indep: dlp | indep: orbit | indep: orbit+absorb | CPU ms per indep: none | dlp | orbit | orbit+absorb |
|---|---|---|---|---|---|---|---|---|---|---|
| 17 | 200 | 5 | 6.6 | 6.2 | 9.2 | 11.0 | 47.057 | 0.275 | 0.181 | 0.156 |
| 19 | 400 | 5 | 27.2 | 25.6 | 26.2 | 29.0 | 10.109 | 0.160 | 0.161 | 0.149 |
| 23 | 450 | 5 | 9.6 | 50.8 | 50.8 | 50.8 | 11.086 | 0.168 | 0.171 | 0.186 |
| 29 | 300 | 5 | 6.0 | 19.2 | 51.6 | 58.4 | 5093.851 | 31.481 | 10.794 | 5.803 |
| 31 | 450 | 5 | 3.2 | 45.6 | 61.6 | 65.6 | 18602.299 | 226.632 | 165.164 | 50.110 |

### Integrity

- receipts: 250; (series, n, cap, arm, seed) groups: 160; verified complete relations: 17865.
- replay or partial-audit failures (linear + curve + audit, all receipts): 0.
- receipts whose recovered scalar equals the planted log: 186; disagreeing with it: 0.
- groups whose repeats disagree on a deterministic counter: 0.
- longest per-cell audit CPU (excluded from online CPU): 0.004 s; setup CPU is excluded from online CPU and is identical across arms.
