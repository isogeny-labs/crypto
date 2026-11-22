# RESULTS (all numbers [pcc:unverified]; local development check)

## Automorphism census (exhaustive Moebius search, linear and semilinear)

| p | seed | l | deg h | rational finite roots | linear maps (incl. id) | nontrivial linear | semilinear | iota = lambda -1 on G,Q |
|---|---|---|---|---|---|---|---|---|
| 503 | 1 | 4049001329390389 | 7 | 1 | 1 | 0 | 0 | True, True |
| 503 | 2 | 4049001288456713 | 7 | 1 | 1 | 0 | 0 | True, True |
| 503 | 3 | 4049001352322749 | 7 | 1 | 1 | 0 | 0 | True, True |
| 1009 | 1 | 263807420145407657 | 7 | 1 | 1 | 0 | 0 | True, True |
| 1009 | 2 | 263807419614957397 | 7 | 1 | 1 | 0 | 0 | True, True |
| 1009 | 3 | 263807419813141697 | 7 | 1 | 1 | 0 | 0 | True, True |
| 1301 | 1 | 1212282365425721801 | 7 | 1 | 1 | 0 | 0 | True, True |
| 1301 | 2 | 1212282366074682901 | 7 | 1 | 1 | 0 | 0 | True, True |
| 1301 | 3 | 1212282366205077161 | 7 | 1 | 1 | 0 | 0 | True, True |
| 1451 | 1 | 2333159791230987941 | 7 | 1 | 1 | 0 | 0 | True, True |
| 1451 | 2 | 2333159789775853813 | 7 | 1 | 1 | 0 | 0 | True, True |
| 1451 | 3 | 2333159789820689941 | 7 | 1 | 1 | 0 | 0 | True, True |
| 1601 | 1 | 4210057237981759961 | 7 | 1 | 1 | 0 | 0 | True, True |
| 1601 | 2 | 4210057235037765977 | 7 | 1 | 1 | 0 | 0 | True, True |
| 1601 | 3 | 4210057235982467833 | 7 | 1 | 1 | 0 | 0 | True, True |

## Random and Kummer alpha census, controls

* p=103 kummer=True: 300 alpha, 300 with a nontrivial linear automorphism (600 maps), 300 with a semilinear map
* p=103 kummer=True: 300 alpha, 300 with a nontrivial linear automorphism (600 maps), 300 with a semilinear map
* p=101 kummer=False: 4000 alpha, 0 with a nontrivial linear automorphism (0 maps), 3 with a semilinear map
* control p=1301: planted x^7+5x total 12 (expected 6); random septic nontrivial 0

## Paired Nagao arm (A = repo baseline rows, Q = orbit-quotient rows from G = {1, iota})

| p | seed | B | rows Q = rows A | relations | unknowns (core) | LA ops | S | S_rel | S_la | S_Q/S_A | S/negrho w=0 | w=0.1 | w=1 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1301 | 1 | 728 | True | 729 | 715 | 13297571 | 0.7288 | 0.7282 | 0.00058 | 1.000 | 0.782 | 0.813 | 0.829 |
| 1301 | 2 | 725 | True | 725 | 713 | 13223299 | 0.7813 | 0.7807 | 0.00058 | 1.000 | 0.838 | 0.872 | 0.889 |
| 1301 | 3 | 730 | True | 731 | 713 | 13223299 | 0.7033 | 0.7027 | 0.00058 | 1.000 | 0.754 | 0.784 | 0.799 |
| 1451 | 1 | 811 | True | 811 | 796 | 16480385 | 0.6015 | 0.6010 | 0.00052 | 1.000 | 0.645 | 0.671 | 0.684 |
| 1451 | 2 | 800 | True | 798 | 788 | 16150849 | 0.6330 | 0.6325 | 0.00051 | 1.000 | 0.679 | 0.706 | 0.720 |
| 1451 | 3 | 804 | True | 804 | 783 | 15946579 | 0.6103 | 0.6098 | 0.00050 | 1.000 | 0.655 | 0.681 | 0.694 |
| 1601 | 1 | 884 | True | 884 | 871 | 19731635 | 0.5233 | 0.5229 | 0.00046 | 1.000 | 0.561 | 0.584 | 0.595 |
| 1601 | 2 | 883 | True | 883 | 867 | 19550851 | 0.5040 | 0.5035 | 0.00046 | 1.000 | 0.541 | 0.562 | 0.573 |
| 1601 | 3 | 883 | True | 882 | 869 | 19641139 | 0.5063 | 0.5058 | 0.00046 | 1.000 | 0.543 | 0.565 | 0.576 |

Per prime geometric means (S/negation-rho, reference 0.9322 U1, rho native 15.81 per sqrt l):

* p=1301 (n=3): w=0 0.791 (range 0.754-0.838), w=0.1 0.822, w=1 0.838
* p=1451 (n=3): w=0 0.660 (range 0.645-0.679), w=0.1 0.686, w=1 0.699
* p=1601 (n=3): w=0 0.548 (range 0.541-0.561), w=0.1 0.570, w=1 0.581

## Arm N (unquotiented signed unknowns), rank analysis on recorded relations (p=251 seed 1, B=153)

* d determined by the plain N system (2B+1 = 307 unknowns): first at None relations (rank 306 of 307 at 420 relations)
* d determined once the B free iota rows are added: first at 160 relations (grid step 10); arm A solved this instance with 154 relations

## Arm N rank analysis on three instances (p=251 seeds 1-3, first 420 relations)

| seed | B | unknowns N | rank plain N at 420 | first k, plain N | first k, N + iota rows | first k, orbit-quotient system (arm A) |
|---|---|---|---|---|---|---|
| 1 | 153 | 307 | 306 of 307 | None | 160 | 160 |
| 2 | 161 | 323 | 322 of 323 | None | 170 | 170 |
| 3 | 161 | 323 | 322 of 323 | None | 170 | 170 |

## Matched negation-map rho on the same instances (rho_matched.py; same l and c_add_e asserted per cell)

Per instance: rho reference = mean of that instance's 12 rho runs; interval = 95% bootstrap over those 12 runs only (the cover run is one deterministic run).

| p | seed | rho steps/sqrt l | rho native/sqrt l | S/rho w=0 [95%] | w=0.1 [95%] | w=1 [95%] |
|---|---|---|---|---|---|---|
| 1301 | 1 | 0.8701 | 14.75 | 0.838 [0.602, 1.248] | 0.871 [0.622, 1.292] | 0.888 [0.638, 1.324] |
| 1301 | 2 | 0.9685 | 16.42 | 0.807 [0.680, 1.031] | 0.839 [0.706, 1.079] | 0.856 [0.720, 1.093] |
| 1301 | 3 | 0.8974 | 15.22 | 0.784 [0.658, 0.938] | 0.815 [0.681, 0.975] | 0.831 [0.694, 0.991] |
| 1451 | 1 | 1.1401 | 19.33 | 0.528 [0.449, 0.645] | 0.549 [0.466, 0.670] | 0.559 [0.475, 0.685] |
| 1451 | 2 | 0.9322 | 15.81 | 0.679 [0.534, 0.937] | 0.706 [0.555, 0.982] | 0.720 [0.564, 0.994] |
| 1451 | 3 | 0.9531 | 16.16 | 0.640 [0.489, 0.875] | 0.666 [0.509, 0.914] | 0.679 [0.519, 0.926] |
| 1601 | 1 | 0.8019 | 13.60 | 0.653 [0.490, 0.950] | 0.679 [0.509, 0.990] | 0.692 [0.521, 1.011] |
| 1601 | 2 | 0.7982 | 13.53 | 0.631 [0.487, 0.884] | 0.657 [0.510, 0.914] | 0.669 [0.517, 0.931] |
| 1601 | 3 | 0.9889 | 16.77 | 0.512 [0.404, 0.665] | 0.532 [0.421, 0.696] | 0.543 [0.426, 0.710] |

Pooled reference (108 rho runs over the 9 instances, steps/sqrt l 0.9278, native 15.73); bootstrap over rho runs and over the 3 instances (too few instances for a trustworthy cover-side interval):

| p | S/rho w=0 [95%] | w=0.1 [95%] | w=1 [95%] |
|---|---|---|---|
| 1301 | 0.794 [0.719, 0.883] | 0.826 [0.747, 0.919] | 0.842 [0.763, 0.936] |
| 1451 | 0.663 [0.605, 0.729] | 0.689 [0.630, 0.758] | 0.702 [0.642, 0.774] |
| 1601 | 0.551 [0.504, 0.606] | 0.573 [0.524, 0.630] | 0.584 [0.535, 0.642] |

(old per-cell table of the abandoned naive runs removed)

