### Per repetition and arm, every attempt charged

| rep | form | arm | attempts | verified | UNSAT | budget or deadline | rank | conflicts | CPU s | conflicts per new independent | CPU s per new independent | dependent verified |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | subgroup_xor | A | 12 | 4 | 8 | 0 | 4/5 | 10304768 | 1091.5 | 2576192 | 272.9 | 0 |
| 1 | subgroup_xor | B | 12 | 12 | 0 | 0 | 5/5 | 3942027 | 1091.0 | 788405 | 218.2 | 7 |
| 2 | subgroup_xor | A | 12 | 4 | 8 | 0 | 4/5 | 10803619 | 1141.4 | 2700905 | 285.3 | 0 |
| 2 | subgroup_xor | B | 12 | 10 | 0 | 2 | 5/5 | 5488419 | 1530.1 | 1097684 | 306.0 | 5 |
| 3 | subgroup_xor | A | 12 | 4 | 8 | 0 | 4/5 | 11814228 | 1319.0 | 2953557 | 329.7 | 0 |
| 3 | subgroup_xor | B | 12 | 12 | 0 | 0 | 5/5 | 4066825 | 1042.1 | 813365 | 208.4 | 7 |

### Ratio B / A of cost per newly independent verified relation

| rep | form | conflicts | CPU | decisions (exploratory) | B below A on conflicts | B below A on CPU |
|---|---|---|---|---|---|---|
| 1 | subgroup_xor | 0.306 | 0.800 | 0.319 | true | true |
| 2 | subgroup_xor | 0.406 | 1.072 | 0.421 | true | false |
| 3 | subgroup_xor | 0.275 | 0.632 | 0.288 | true | true |
| mean | subgroup_xor | 0.329 (range 0.275 to 0.406) | 0.835 (range 0.632 to 1.072) | | | |

### Decision rule (PROTOCOL.md)

- subgroup_xor: B below A on both clocks in 2 of 3 repetitions; at or above on both in 0; verdict: mixed (the clocks or repetitions disagree), counts as not supported
- every relation passed every check, no rejected tuple: true
- **overall: H1 not supported**

### Where the cost goes, by outcome (conflicts; CPU s)

| rep | arm | verified (n, conflicts, CPU s) | UNSAT (n, conflicts, CPU s) | budget or deadline (n, conflicts, CPU s) |
|---|---|---|---|---|
| 1 | A | 4, 445601, 21.8 | 8, 9859167, 1069.7 | 0, 0, 0.0 |
| 1 | B | 12, 3942027, 1091.0 | 0, 0, 0.0 | 0, 0, 0.0 |
| 2 | A | 4, 585507, 27.7 | 8, 10218112, 1113.7 | 0, 0, 0.0 |
| 2 | B | 10, 2919429, 740.8 | 0, 0, 0.0 | 2, 2568990, 789.3 |
| 3 | A | 4, 867451, 45.6 | 8, 10946777, 1273.4 | 0, 0, 0.0 |
| 3 | B | 12, 4066825, 1042.1 | 0, 0, 0.0 | 0, 0, 0.0 |

### Like for like: the four targets both arms decompose (b0 to b3)

| rep | A conflicts | B conflicts | B/A conflicts | A CPU s | B CPU s | B/A CPU |
|---|---|---|---|---|---|---|
| 1 | 445601 | 962366 | 2.16 | 21.8 | 266.5 | 12.24 |
| 2 | 585507 | 1200094 | 2.05 | 27.7 | 301.1 | 10.87 |
| 3 | 867451 | 1165738 | 1.34 | 45.6 | 297.2 | 6.52 |

### Per target, median over repetitions

| form | target | A status | A conflicts | A CPU s | B status | B conflicts | B CPU s | B/A conflicts | B/A CPU |
|---|---|---|---|---|---|---|---|---|---|
| subgroup_xor | a0 | unsat | 1585758 | 197.4 | verified | 420752 | 103.1 | - | - |
| subgroup_xor | a1 | unsat | 1483141 | 175.9 | verified | 120873 | 29.6 | - | - |
| subgroup_xor | a2 | unsat | 1270254 | 150.4 | deadline/verified | 473546 | 128.3 | - | - |
| subgroup_xor | a3 | unsat | 757151 | 57.9 | deadline/verified | 374065 | 96.3 | - | - |
| subgroup_xor | b0 | verified | 141081 | 6.4 | verified | 38346 | 11.1 | 0.27 | 1.72 |
| subgroup_xor | b1 | verified | 102075 | 4.6 | verified | 203431 | 58.5 | 1.99 | 12.84 |
| subgroup_xor | b2 | verified | 88797 | 4.2 | verified | 373090 | 97.0 | 4.20 | 23.18 |
| subgroup_xor | b3 | verified | 114709 | 5.3 | verified | 329223 | 76.2 | 2.87 | 14.35 |
| subgroup_xor | c0 | unsat | 1634026 | 193.6 | verified | 442204 | 107.5 | - | - |
| subgroup_xor | c1 | unsat | 1377629 | 144.1 | verified | 163986 | 40.1 | - | - |
| subgroup_xor | c2 | unsat | 1088955 | 100.2 | verified | 88438 | 22.9 | - | - |
| subgroup_xor | c3 | unsat | 1658140 | 228.4 | verified | 823197 | 220.4 | - | - |

### Cost charged when each rank was first reached (attempts, conflicts, CPU s); depends on the schedule's target order

| rep | form | arm | rank 1 | rank 2 | rank 3 | rank 4 | rank 5 |
|---|---|---|---|---|---|---|---|
| 1 | subgroup_xor | A | 5, 4667999, 482.5 | 6, 4730745, 485.5 | 7, 4996237, 499.0 | 8, 5015481, 499.6 | - |
| 1 | subgroup_xor | B | 1, 18411, 5.3 | 2, 856934, 242.9 | 3, 875909, 247.9 | 4, 1249974, 344.2 | 5, 1288320, 355.2 |
| 2 | subgroup_xor | A | 5, 5322491, 592.5 | 6, 5424566, 597.0 | 7, 5507730, 600.6 | 8, 5766917, 613.7 | - |
| 2 | subgroup_xor | B | 1, 420752, 103.1 | 2, 541625, 132.8 | 5, 3566901, 1036.1 | 6, 3665076, 1058.8 | 7, 3878645, 1112.8 |
| 3 | subgroup_xor | A | 5, 5423586, 623.5 | 6, 5702280, 637.8 | 7, 5791077, 642.0 | 8, 5905786, 647.3 | - |
| 3 | subgroup_xor | B | 1, 609515, 164.8 | 2, 701503, 187.0 | 3, 1175049, 315.2 | 4, 1215757, 325.2 | 5, 1234548, 329.8 |
