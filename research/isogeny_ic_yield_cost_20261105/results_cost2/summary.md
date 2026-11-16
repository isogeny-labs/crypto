## Rho baselines (mean over curves; 100 targets per curve)

| bits | curves | q (curve 0) | Brent, no neg | DP+neg, 1 DLP (incl setup) | DP+neg walk only | DP+neg multi-target, per DLP at D=100 | sqrt(pi q/2) |
|---|---|---|---|---|---|---|---|
| 14 | 3 | 2069 | 1080 | 1710 | 780 | 78 | 57 |
| 16 | 3 | 8179 | 1358 | 2089 | 950 | 105 | 113 |
| 18 | 3 | 32869 | 1752 | 2218 | 891 | 103 | 227 |
| 20 | 3 | 131267 | 2311 | 3243 | 1724 | 168 | 454 |
| 22 | 3 | 524053 | 3199 | 4057 | 2340 | 245 | 907 |
| 24 | 3 | 2097643 | 4814 | 4757 | 2863 | 338 | 1815 |
| 26 | 3 | 8389657 | 7798 | 7508 | 5413 | 573 | 3630 |
| 28 | 3 | 33550799 | 14184 | 12017 | 9722 | 1016 | 7260 |
| 30 | 3 | 134222453 | 25221 | 20058 | 17576 | 1956 | 14520 |
| 32 | 3 | 536866381 | 48936 | 35079 | 32412 | 3771 | 29040 |

## Scaling exponents, cost ∝ q^e (bits ≥ 20; curve-bootstrap 95% CI; fixed overhead subtracted for IC)

| series | e (95% CI) | sizes |
|---|---|---|
| rho Brent no-neg (incl. 64-point setup) | 0.370 (0.365–0.375) | 20–32 |
| rho Brent no-neg walk only | 0.500 (0.491–0.511) | 20–32 |
| rho DP+neg walk | 0.362 (0.344–0.387) | 20–32 |
| rho multi-target per DLP, D=100 | 0.376 (0.363–0.393) | 20–32 |
| IC FULL m=2 c=0.5 coset=off D=1 | 0.975 (0.956–0.997) | 20–24 |
| IC FULL m=2 c=0.5 coset=off D=100 | 0.967 (0.949–0.989) | 20–24 |
| IC FULL m=2 c=0.5 coset=folded D=1 | 0.976 (0.956–1.002) | 20–24 |
| IC FULL m=2 c=0.5 coset=folded D=100 | 0.969 (0.950–0.994) | 20–24 |
| IC FULL m=3 c=0.5 coset=off D=1 | 0.840 (0.821–0.858) | 20–28 |
| IC FULL m=3 c=0.5 coset=off D=100 | 0.819 (0.801–0.835) | 20–28 |
| IC FULL m=3 c=0.5 coset=folded D=1 | 0.882 (0.830–1.021) | 20–28 |
| IC FULL m=3 c=0.5 coset=folded D=100 | 0.857 (0.815–0.943) | 20–28 |
| IC FULL m=3 c=1.0 coset=off D=1 | 0.970 (0.896–1.040) | 20–24 |
| IC FULL m=3 c=1.0 coset=off D=100 | 0.966 (0.893–1.035) | 20–24 |
| IC FULL m=3 c=1.0 coset=folded D=1 | 0.970 (0.896–1.040) | 20–24 |
| IC FULL m=3 c=1.0 coset=folded D=100 | 0.966 (0.893–1.035) | 20–24 |
| IC FULL m=4 c=0.5 coset=folded D=1 | 0.821 (0.763–0.870) | 20–30 |
| IC FULL m=4 c=0.5 coset=folded D=100 | 0.743 (0.690–0.781) | 20–30 |
| IC FULL m=4 c=1.0 coset=off D=1 | 0.835 (0.695–1.194) | 20–26 |
| IC FULL m=4 c=1.0 coset=off D=100 | 0.819 (0.690–1.038) | 20–26 |
| IC FULL m=4 c=1.0 coset=folded D=1 | 0.835 (0.695–1.199) | 20–26 |
| IC FULL m=4 c=1.0 coset=folded D=100 | 0.825 (0.691–1.102) | 20–26 |
| IC FULL m=5 c=0.5 coset=folded D=1 | 0.643 (0.643–0.643) | 20–26 |
| IC FULL m=5 c=0.5 coset=folded D=100 | 0.440 (0.440–0.440) | 20–26 |
| IC FULL m=5 c=1.0 coset=off D=1 | 0.739 (0.704–0.782) | 20–28 |
| IC FULL m=5 c=1.0 coset=off D=100 | 0.723 (0.692–0.760) | 20–28 |
| IC FULL m=5 c=1.0 coset=folded D=1 | 0.722 (0.591–0.919) | 20–28 |
| IC FULL m=5 c=1.0 coset=folded D=100 | 0.681 (0.575–0.763) | 20–28 |
| IC MITM m=2 c=0.5 coset=off D=1 | 1.031 (0.964–1.120) | 20–24 |
| IC MITM m=2 c=0.5 coset=off D=100 | 0.981 (0.921–1.056) | 20–24 |
| IC MITM m=2 c=0.5 coset=folded D=1 | 1.098 (1.018–1.182) | 20–24 |
| IC MITM m=2 c=0.5 coset=folded D=100 | 1.036 (0.969–1.107) | 20–24 |
| IC MITM m=2 c=1.0 coset=off D=1 | 1.138 (1.071–1.211) | 20–24 |
| IC MITM m=2 c=1.0 coset=off D=100 | 1.109 (1.049–1.174) | 20–24 |
| IC MITM m=2 c=1.0 coset=folded D=1 | 1.108 (0.976–1.264) | 20–24 |
| IC MITM m=2 c=1.0 coset=folded D=100 | 1.071 (0.952–1.192) | 20–24 |
| IC MITM m=2 c=2.0 coset=off D=1 | 1.112 (1.008–1.223) | 20–24 |
| IC MITM m=2 c=2.0 coset=off D=100 | 1.099 (0.999–1.206) | 20–24 |
| IC MITM m=2 c=2.0 coset=folded D=1 | 1.070 (0.999–1.133) | 20–24 |
| IC MITM m=2 c=2.0 coset=folded D=100 | 1.049 (0.975–1.109) | 20–24 |
| IC MITM m=3 c=0.5 coset=off D=1 | 0.685 (0.664–0.709) | 20–32 |
| IC MITM m=3 c=0.5 coset=off D=100 | 0.554 (0.541–0.568) | 20–32 |
| IC MITM m=3 c=0.5 coset=folded D=1 | 0.677 (0.647–0.712) | 20–32 |
| IC MITM m=3 c=0.5 coset=folded D=100 | 0.524 (0.489–0.563) | 20–32 |
| IC MITM m=3 c=1.0 coset=off D=1 | 0.700 (0.686–0.713) | 20–32 |
| IC MITM m=3 c=1.0 coset=off D=100 | 0.597 (0.584–0.608) | 20–32 |
| IC MITM m=3 c=1.0 coset=folded D=1 | 0.673 (0.664–0.682) | 20–32 |
| IC MITM m=3 c=1.0 coset=folded D=100 | 0.607 (0.598–0.615) | 20–32 |
| IC MITM m=3 c=2.0 coset=off D=1 | 0.680 (0.674–0.687) | 20–30 |
| IC MITM m=3 c=2.0 coset=off D=100 | 0.639 (0.630–0.651) | 20–30 |
| IC MITM m=3 c=2.0 coset=folded D=1 | 0.655 (0.641–0.669) | 20–30 |
| IC MITM m=3 c=2.0 coset=folded D=100 | 0.638 (0.625–0.649) | 20–30 |
| IC MITM m=4 c=0.5 coset=folded D=1 | 0.695 (0.669–0.725) | 20–32 |
| IC MITM m=4 c=0.5 coset=folded D=100 | 0.595 (0.580–0.610) | 20–32 |
| IC MITM m=4 c=1.0 coset=off D=1 | 0.736 (0.694–0.788) | 20–32 |
| IC MITM m=4 c=1.0 coset=off D=100 | 0.571 (0.524–0.626) | 20–32 |
| IC MITM m=4 c=1.0 coset=folded D=1 | 0.669 (0.618–0.740) | 20–32 |
| IC MITM m=4 c=1.0 coset=folded D=100 | 0.559 (0.510–0.622) | 20–32 |
| IC MITM m=4 c=2.0 coset=off D=1 | 0.752 (0.729–0.774) | 20–32 |
| IC MITM m=4 c=2.0 coset=off D=100 | 0.633 (0.601–0.668) | 20–32 |
| IC MITM m=4 c=2.0 coset=folded D=1 | 0.711 (0.682–0.739) | 20–32 |
| IC MITM m=4 c=2.0 coset=folded D=100 | 0.606 (0.574–0.640) | 20–32 |
| IC MITM m=4 c=4.0 coset=off D=1 | 0.741 (0.713–0.768) | 20–30 |
| IC MITM m=4 c=4.0 coset=off D=100 | 0.679 (0.660–0.699) | 20–30 |
| IC MITM m=4 c=4.0 coset=folded D=1 | 0.583 (0.568–0.596) | 20–30 |
| IC MITM m=4 c=4.0 coset=folded D=100 | 0.571 (0.558–0.583) | 20–30 |
| IC MITM m=4 c=8.0 coset=off D=1 | 0.578 (0.570–0.590) | 20–28 |
| IC MITM m=4 c=8.0 coset=off D=100 | 0.579 (0.572–0.588) | 20–28 |
| IC MITM m=4 c=8.0 coset=folded D=1 | 0.479 (0.467–0.494) | 20–28 |
| IC MITM m=4 c=8.0 coset=folded D=100 | 0.474 (0.463–0.488) | 20–28 |
| IC MITM m=5 c=0.5 coset=folded D=1 | 0.436 (0.436–0.436) | 20–26 |
| IC MITM m=5 c=0.5 coset=folded D=100 | 0.506 (0.506–0.506) | 20–26 |
| IC MITM m=5 c=1.0 coset=off D=1 | 0.612 (0.587–0.642) | 20–32 |
| IC MITM m=5 c=1.0 coset=off D=100 | 0.501 (0.479–0.530) | 20–32 |
| IC MITM m=5 c=1.0 coset=folded D=1 | 0.575 (0.518–0.688) | 20–32 |
| IC MITM m=5 c=1.0 coset=folded D=100 | 0.463 (0.405–0.550) | 20–32 |
| IC MITM m=5 c=2.0 coset=off D=1 | 0.554 (0.517–0.600) | 20–32 |
| IC MITM m=5 c=2.0 coset=off D=100 | 0.438 (0.380–0.526) | 20–32 |
| IC MITM m=5 c=2.0 coset=folded D=1 | 0.551 (0.534–0.567) | 20–32 |
| IC MITM m=5 c=2.0 coset=folded D=100 | 0.471 (0.417–0.539) | 20–32 |
| IC MITM m=5 c=4.0 coset=off D=1 | 0.570 (0.532–0.619) | 20–30 |
| IC MITM m=5 c=4.0 coset=off D=100 | 0.554 (0.527–0.581) | 20–30 |
| IC MITM m=5 c=4.0 coset=folded D=1 | 0.569 (0.523–0.631) | 20–30 |
| IC MITM m=5 c=4.0 coset=folded D=100 | 0.563 (0.519–0.621) | 20–30 |
| IC MITM m=5 c=8.0 coset=off D=1 | 0.537 (0.516–0.556) | 20–26 |
| IC MITM m=5 c=8.0 coset=off D=100 | 0.536 (0.515–0.555) | 20–26 |
| IC MITM m=5 c=8.0 coset=folded D=1 | 0.537 (0.516–0.556) | 20–26 |
| IC MITM m=5 c=8.0 coset=folded D=100 | 0.535 (0.515–0.555) | 20–26 |

## Best IC configuration per size (config must have run on every curve of that size; chosen by mean cost over curves)

| bits | D | best config | IC ops/DLP | ÷ rho Brent | ÷ rho DP+neg (1 target) | ÷ rho multi-target DP+neg (D=100) | memory (table+list) | curves |
|---|---|---|---|---|---|---|---|---|
| 14 | 1 | MITM m=3 c=1.0 coset=folded | 1848 | 1.71× | 1.08× | 23.70× | 307 | 3 |
| 14 | 100 | MITM m=3 c=2.0 coset=folded | 62 | 0.06× | 0.04× | 0.80× | 944 | 3 |
| 16 | 1 | MITM m=3 c=1.0 coset=folded | 2970 | 2.19× | 1.42× | 28.26× | 1011 | 3 |
| 16 | 100 | MITM m=3 c=1.0 coset=folded | 86 | 0.06× | 0.04× | 0.82× | 1011 | 3 |
| 18 | 1 | FULL m=4 c=0.5 coset=folded | 5519 | 3.15× | 2.49× | 53.72× | 1280 | 3 |
| 18 | 100 | MITM m=3 c=1.0 coset=folded | 145 | 0.08× | 0.07× | 1.41× | 1928 | 3 |
| 20 | 1 | MITM m=3 c=1.0 coset=folded | 10909 | 4.72× | 3.36× | 64.95× | 5724 | 3 |
| 20 | 100 | MITM m=3 c=1.0 coset=folded | 217 | 0.09× | 0.07× | 1.29× | 5724 | 3 |
| 22 | 1 | MITM m=3 c=1.0 coset=folded | 23746 | 7.42× | 5.85× | 97.00× | 13750 | 3 |
| 22 | 100 | MITM m=3 c=1.0 coset=folded | 368 | 0.12× | 0.09× | 1.50× | 13750 | 3 |
| 24 | 1 | MITM m=5 c=2.0 coset=folded | 57121 | 11.87× | 12.01× | 168.76× | 35715 | 3 |
| 24 | 100 | MITM m=3 c=1.0 coset=folded | 792 | 0.16× | 0.17× | 2.34× | 35329 | 3 |
| 26 | 1 | MITM m=5 c=2.0 coset=folded | 111572 | 14.31× | 14.86× | 194.72× | 58805 | 3 |
| 26 | 100 | MITM m=3 c=1.0 coset=folded | 1888 | 0.24× | 0.25× | 3.30× | 78617 | 3 |
| 28 | 1 | MITM m=5 c=2.0 coset=folded | 301352 | 21.25× | 25.08× | 296.48× | 195483 | 3 |
| 28 | 100 | MITM m=3 c=1.0 coset=folded | 4403 | 0.31× | 0.37× | 4.33× | 191395 | 3 |
| 30 | 1 | MITM m=5 c=2.0 coset=folded | 599636 | 23.78× | 29.89× | 306.57× | 408323 | 3 |
| 30 | 100 | MITM m=5 c=2.0 coset=folded | 8196 | 0.32× | 0.41× | 4.19× | 408323 | 3 |
| 32 | 1 | MITM m=5 c=2.0 coset=folded | 1401979 | 28.65× | 39.97× | 371.82× | 995603 | 3 |
| 32 | 100 | MITM m=5 c=2.0 coset=folded | 17929 | 0.37× | 0.51× | 4.76× | 995603 | 3 |

## Best MITM per m and size, D=1, ratio to single-target rho DP+neg (mean over curves)

| bits | m=2 | m=3 | m=4 | m=5 |
|---|---|---|---|---|
| 14 | 1.8× (c=2.0,f) | 1.1× (c=1.0,f) | 1.3× (c=2.0,f) | – |
| 16 | 4.6× (c=1.0,f) | 1.4× (c=1.0,f) | 1.8× (c=2.0,f) | – |
| 18 | 18.1× (c=1.0,f) | 2.5× (c=1.0,f) | 3.0× (c=2.0,f) | 2.8× (c=2.0,f) |
| 20 | 45.7× (c=2.0,f) | 3.4× (c=1.0,f) | 4.3× (c=2.0,f) | 5.5× (c=2.0,f) |
| 22 | 165.6× (c=2.0,f) | 5.9× (c=1.0,f) | 7.7× (c=2.0,f) | 6.7× (c=2.0,f) |
| 24 | 598.6× (c=2.0,f) | 12.4× (c=1.0,f) | 16.6× (c=2.0,f) | 12.0× (c=2.0,f) |
| 26 | – | 19.5× (c=0.5,f) | 27.2× (c=4.0,f) | 14.9× (c=2.0,f) |
| 28 | – | 32.6× (c=1.0,f) | 42.0× (c=4.0,f) | 25.1× (c=2.0,f) |
| 30 | – | 49.5× (c=1.0,f) | 59.5× (c=4.0,f) | 29.9× (c=2.0,f) |
| 32 | – | 67.7× (c=1.0,f) | 123.5× (c=2.0,f) | 40.0× (c=2.0,f) |

## Folded-coset gain, measured: cost(coset off) / cost(folded), D=1, mean over curves

| strat | m | c | 14 | 16 | 18 | 20 | 22 | 24 | 26 | 28 | 30 | 32 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| FULL | 2 | 0.5 | 1.0 | 1.0 | 1.0 | 1.0 | 1.0 | 1.0 | – | – | – | – |
| FULL | 2 | 1.0 | 1.0 | 1.0 | 1.0 | 1.0 | 1.0 | – | – | – | – | – |
| FULL | 2 | 2.0 | 1.0 | 1.0 | 1.0 | 1.0 | – | – | – | – | – | – |
| FULL | 3 | 0.5 | 1.3 | 1.1 | 1.1 | 1.3 | 1.0 | 1.0 | 1.0 | 1.0 | – | – |
| FULL | 3 | 1.0 | 1.0 | 1.0 | 1.0 | 1.0 | 1.0 | 1.0 | – | – | – | – |
| FULL | 3 | 2.0 | 1.0 | 1.0 | 1.0 | 1.0 | – | – | – | – | – | – |
| FULL | 4 | 0.5 | – | 1.4 | – | 1.1 | – | – | – | – | – | – |
| FULL | 4 | 1.0 | 1.0 | 1.2 | 1.0 | 1.0 | 1.0 | 1.0 | 1.0 | – | – | – |
| FULL | 4 | 2.0 | 1.0 | 1.1 | 1.0 | 1.0 | 1.0 | – | – | – | – | – |
| FULL | 5 | 0.5 | – | – | – | – | – | – | – | – | – | – |
| FULL | 5 | 1.0 | – | – | 1.0 | 1.4 | 1.0 | 1.3 | 1.3 | 1.4 | – | – |
| FULL | 5 | 2.0 | – | – | 1.0 | 1.0 | 1.0 | – | – | – | – | – |
| MITM | 2 | 0.5 | 7.4 | 7.4 | 6.8 | 10.0 | 6.9 | 8.4 | – | – | – | – |
| MITM | 2 | 1.0 | 5.0 | 10.6 | 10.0 | 11.1 | 7.9 | 12.2 | – | – | – | – |
| MITM | 2 | 2.0 | 8.4 | 9.1 | 10.8 | 12.0 | 11.8 | 13.6 | – | – | – | – |
| MITM | 3 | 0.5 | 5.1 | 4.4 | 5.3 | 6.3 | 8.2 | 5.7 | 9.4 | 8.6 | 6.4 | 8.3 |
| MITM | 3 | 1.0 | 2.2 | 2.8 | 3.3 | 3.5 | 4.1 | 4.8 | 4.7 | 4.4 | 4.5 | 5.5 |
| MITM | 3 | 2.0 | 1.2 | 1.3 | 1.3 | 1.4 | 1.6 | 1.6 | 1.8 | 1.9 | 1.6 | – |
| MITM | 4 | 0.5 | – | 4.3 | – | 4.3 | – | – | – | – | – | – |
| MITM | 4 | 1.0 | 5.7 | 9.1 | 6.9 | 5.2 | 5.7 | 10.2 | 8.5 | 8.6 | 8.1 | 10.8 |
| MITM | 4 | 2.0 | 3.9 | 3.8 | 6.6 | 7.5 | 8.0 | 10.8 | 11.4 | 9.7 | 11.3 | 12.2 |
| MITM | 4 | 4.0 | – | – | 1.9 | 2.0 | 2.4 | 3.0 | 4.7 | 4.8 | 5.9 | – |
| MITM | 4 | 8.0 | – | – | 1.1 | 1.1 | 1.2 | 1.3 | 1.5 | 1.9 | – | – |
| MITM | 5 | 0.5 | – | – | – | – | – | – | – | – | – | – |
| MITM | 5 | 1.0 | – | – | 6.4 | 3.0 | 6.3 | 10.8 | 7.8 | 6.7 | 5.5 | 6.7 |
| MITM | 5 | 2.0 | – | – | 4.2 | 3.4 | 3.3 | 4.1 | 5.0 | 3.4 | 3.9 | 3.8 |
| MITM | 5 | 4.0 | – | – | 1.0 | 1.1 | 1.1 | 1.1 | 1.1 | 1.1 | 1.1 | – |
| MITM | 5 | 8.0 | – | – | 1.0 | 1.0 | 1.0 | 1.0 | 1.0 | – | – | – |

## Checks
- IC: 1001 configs, 100100/100100 scalars recovered, 2 wrong candidates (rejected by verification).
- Rho: 30 (size, curve) cells × 100 targets × 3 methods = 9000 DLPs, all asserted correct.
- Nonzero rank deficiencies (m, coset, dims): [(2, 0, 2), (2, 0, 3), (2, 1, 1)]
