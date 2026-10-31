### Verdict under the preregistered rule (PROTOCOL_ADDENDUM_2.md)

Thresholds on the per-repetition median CPU of the eight arm A refutations: solver if at or below 45 s with all eight UNSAT, encoding if at or above 80 s, intermediate between.

| rep | complete | median CPU s | min | max | mean | not UNSAT (of 8) | class |
|---|---|---|---|---|---|---|---|
| 1 | true | 36.14 | 19.94 | 50.52 | 35.77 | 0 | S |
| 2 | true | 20.83 | 15.48 | 26.72 | 20.25 | 0 | S |
| 3 | true | 24.85 | 21.21 | 35.21 | 25.89 | 0 | S |

**Verdict: SOLVER** (3 complete repetition(s)).

### Arm A refutations, CPU seconds per target

Legacy: CryptoMiniSat 5.16.0 via pycryptosat on the legacy Python encoding, one trial. Native: the crate's `sat::Solver` on this study's encoding. CMS: the pinned binary on this study's encoding exported with `to_dimacs_xor`. Status other than `unsat` is marked; a budget or deadline value is a lower bound.

| target | legacy | native rep 1 | native rep 2 | native rep 3 | CMS rep 1 | CMS rep 2 | CMS rep 3 |
|---|---|---|---|---|---|---|---|
| a0 | 8.3 | 122.5 | 202.3 | 197.4 | 48.6 | 26.7 | 27.3 |
| a1 | 23.4 | 98.2 | 175.9 | 215.6 | 19.9 | 16.0 | 29.8 |
| a2 | 20.6 | 176.2 | 149.8 | 150.4 | 50.5 | 22.6 | 21.8 |
| a3 | 15.0 | 80.9 | 57.9 | 38.2 | 27.6 | 23.5 | 21.2 |
| c0 | 44.0 | 79.3 | 215.5 | 193.6 | 22.5 | 15.5 | 35.2 |
| c1 | 29.3 | 180.5 | 83.1 | 144.1 | 31.6 | 16.1 | 22.1 |
| c2 | 28.0 | 103.7 | 79.1 | 100.2 | 44.7 | 21.8 | 23.0 |
| c3 | 24.2 | 228.4 | 150.0 | 233.7 | 40.7 | 19.9 | 26.7 |
| median | 23.8 | 113.1 | 149.9 | 172.0 | 36.1 | 20.8 | 24.8 |

### Paired comparison per repetition, every attempt charged

Cost per newly independent verified relation, B over A. Legacy B/A CPU: 1.61 (one trial).

| rep | CMS conflicts B/A | CMS CPU B/A | native conflicts B/A | native CPU B/A | CMS arm A CPU s | CMS arm B CPU s | CMS unknown (budget/deadline) A, B |
|---|---|---|---|---|---|---|---|
| 1 | 0.938 | 1.177 | 0.306 | 0.800 | 295.6 | 435.0 | 0, 0 |
| 2 | 1.182 | 1.489 | 0.406 | 1.072 | 190.6 | 354.6 | 0, 0 |
| 3 | 1.167 | 1.488 | 0.275 | 0.632 | 237.3 | 441.3 | 0, 0 |

Direction on CPU per new relation (B/A above 1 is the legacy direction: 1.61): read the CMS column above; a repetition with arm B skipped has no ratio.

### Full tables, same renderer as the native study

The block headed "Decision rule (PROTOCOL.md)" below applies the native study's H1 rule to this solver's data. It is exploratory here: the preregistered rule of this follow-up is the verdict at the top.

### Per repetition and arm, every attempt charged

| rep | form | arm | attempts | verified | UNSAT | budget or deadline | rank | conflicts | CPU s | conflicts per new independent | CPU s per new independent | dependent verified |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | subgroup_xor | A | 12 | 4 | 8 | 0 | 4/5 | 8277579 | 295.6 | 2069395 | 73.9 | 0 |
| 1 | subgroup_xor | B | 12 | 12 | 0 | 0 | 5/5 | 9706614 | 435.0 | 1941323 | 87.0 | 7 |
| 2 | subgroup_xor | A | 12 | 4 | 8 | 0 | 4/5 | 5515296 | 190.6 | 1378824 | 47.6 | 0 |
| 2 | subgroup_xor | B | 12 | 12 | 0 | 0 | 5/5 | 8147317 | 354.6 | 1629463 | 70.9 | 7 |
| 3 | subgroup_xor | A | 12 | 4 | 8 | 0 | 4/5 | 6662978 | 237.3 | 1665744 | 59.3 | 0 |
| 3 | subgroup_xor | B | 12 | 12 | 0 | 0 | 5/5 | 9717605 | 441.3 | 1943521 | 88.3 | 7 |

### Ratio B / A of cost per newly independent verified relation

| rep | form | conflicts | CPU | decisions (exploratory) | B below A on conflicts | B below A on CPU |
|---|---|---|---|---|---|---|
| 1 | subgroup_xor | 0.938 | 1.177 | 1.386 | true | false |
| 2 | subgroup_xor | 1.182 | 1.489 | 1.958 | false | false |
| 3 | subgroup_xor | 1.167 | 1.488 | 1.624 | false | false |
| mean | subgroup_xor | 1.096 (range 0.938 to 1.182) | 1.384 (range 1.177 to 1.489) | | | |

### Decision rule (PROTOCOL.md)

- subgroup_xor: B below A on both clocks in 0 of 3 repetitions; at or above on both in 2; verdict: mixed (the clocks or repetitions disagree), counts as not supported
- every relation passed every check, no rejected tuple: true
- **overall: H1 not supported**

### Where the cost goes, by outcome (conflicts; CPU s)

| rep | arm | verified (n, conflicts, CPU s) | UNSAT (n, conflicts, CPU s) | budget or deadline (n, conflicts, CPU s) |
|---|---|---|---|---|
| 1 | A | 4, 331957, 9.5 | 8, 7945622, 286.1 | 0, 0, 0.0 |
| 1 | B | 12, 9706614, 435.0 | 0, 0, 0.0 | 0, 0, 0.0 |
| 2 | A | 4, 889066, 28.6 | 8, 4626230, 162.0 | 0, 0, 0.0 |
| 2 | B | 12, 8147317, 354.6 | 0, 0, 0.0 | 0, 0, 0.0 |
| 3 | A | 4, 885547, 30.2 | 8, 5777431, 207.1 | 0, 0, 0.0 |
| 3 | B | 12, 9717605, 441.3 | 0, 0, 0.0 | 0, 0, 0.0 |

### Like for like: the four targets both arms decompose (b0 to b3)

| rep | A conflicts | B conflicts | B/A conflicts | A CPU s | B CPU s | B/A CPU |
|---|---|---|---|---|---|---|
| 1 | 331957 | 3137998 | 9.45 | 9.5 | 141.6 | 14.90 |
| 2 | 889066 | 2684815 | 3.02 | 28.6 | 114.4 | 4.00 |
| 3 | 885547 | 2649460 | 2.99 | 30.2 | 121.2 | 4.01 |

### Per target, median over repetitions

| form | target | A status | A conflicts | A CPU s | B status | B conflicts | B CPU s | B/A conflicts | B/A CPU |
|---|---|---|---|---|---|---|---|---|---|
| subgroup_xor | a0 | unsat | 822757 | 27.3 | verified | 765001 | 33.6 | - | - |
| subgroup_xor | a1 | unsat | 559309 | 19.9 | verified | 878764 | 41.8 | - | - |
| subgroup_xor | a2 | unsat | 605390 | 22.6 | verified | 718864 | 28.5 | - | - |
| subgroup_xor | a3 | unsat | 655962 | 23.5 | verified | 620445 | 27.9 | - | - |
| subgroup_xor | b0 | verified | 133512 | 4.3 | verified | 826795 | 32.1 | 6.19 | 7.46 |
| subgroup_xor | b1 | verified | 80807 | 2.3 | verified | 667600 | 27.1 | 8.26 | 11.81 |
| subgroup_xor | b2 | verified | 143433 | 5.5 | verified | 752599 | 33.8 | 5.25 | 6.14 |
| subgroup_xor | b3 | verified | 89736 | 2.8 | verified | 776819 | 36.2 | 8.66 | 12.88 |
| subgroup_xor | c0 | unsat | 684686 | 22.5 | verified | 979268 | 41.4 | - | - |
| subgroup_xor | c1 | unsat | 637252 | 22.1 | verified | 800761 | 39.8 | - | - |
| subgroup_xor | c2 | unsat | 641958 | 23.0 | verified | 645242 | 29.0 | - | - |
| subgroup_xor | c3 | unsat | 735128 | 26.7 | verified | 685226 | 31.4 | - | - |

### Cost charged when each rank was first reached (attempts, conflicts, CPU s); depends on the schedule's target order

| rep | form | arm | rank 1 | rank 2 | rank 3 | rank 4 | rank 5 |
|---|---|---|---|---|---|---|---|
| 1 | subgroup_xor | A | 5, 3992205, 148.4 | 6, 4073012, 150.7 | 7, 4191979, 154.5 | 8, 4258589, 156.2 | - |
| 1 | subgroup_xor | B | 1, 765001, 33.6 | 2, 1643765, 75.5 | 3, 2362629, 104.0 | 4, 3319742, 151.3 | 5, 4173152, 193.6 |
| 2 | subgroup_xor | A | 5, 2693338, 93.0 | 6, 2767453, 95.3 | 7, 3359156, 114.5 | 8, 3448892, 117.3 | - |
| 2 | subgroup_xor | B | 1, 770979, 35.4 | 2, 1533759, 70.9 | 3, 2054845, 94.7 | 4, 2665022, 120.9 | 5, 3491817, 152.9 |
| 3 | subgroup_xor | A | 5, 3088003, 112.1 | 6, 3175247, 114.7 | 7, 3318680, 120.2 | 8, 3635044, 130.3 | - |
| 3 | subgroup_xor | B | 1, 524554, 25.5 | 2, 1563074, 71.4 | 3, 2384919, 106.6 | 4, 3005364, 134.5 | 6, 4348789, 194.1 |
