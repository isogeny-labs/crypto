# P2 (ECCp-131 scaled prime-field analogues): main-session reading

Two conductor-1 prime-field classes (D = -84171, h = 128, class A; D = -414651, h = 300, class B),
each enumerated in full (Hilbert class polynomial roots), at 24, 28 and 32 bits, m = 2 and 3.
Instrument: ecbench `ic.pipeline`, prime-abscissa factor base of fixed size N per cell, mitm
decomposition; primary metric relation yield per attempt (exact counts). Each cell was audited
(`ecbench verify --replay 12`, all OK) and analysed with the frozen `p2tool analyze`
(PROTOCOL.md §5–6). Local route, isolation L0 (amendment A1); 36- and 40-bit cells deferred.
Bonferroni family 20 tests, alpha 0.0025.

| cell | yield/attempt | ICC (perm p; boot. upper 95%) | max abs z (perm p) | max/min | iso RMS log | planted 20% power (ICC) | verdict (PROTOCOL §6) |
|---|---|---|---|---|---|---|---|
| c24A_m2 | 0.0199 | -0.0103 (0.901; 0.074) | 2.71 (0.5847) | 1.081 | 0.0336 | 1.00 | null supported |
| c24A_m3 | 0.0330 | -0.0049 (0.896; 0.038) | 2.31 (0.9285) | 1.130 | 0.0481 | 0.38 | not informative at 20% |
| c24B_m2 | 0.0198 | -0.0052 (0.830; 0.073) | 2.63 (0.9135) | 1.097 | 0.0167 | 0.99 | null supported |
| c24B_m3 | 0.0340 | 0.0014 (0.281; 0.041) | 2.75 (0.8266) | 1.156 | 0.0361 | 0.84 | not informative at 20% |
| c28A_m2 | 0.0199 | 0.0143 (0.054; 0.103) | 3.24 (0.1489) | 1.115 | 0.0202 | 1.00 | null supported |
| c28A_m3 | 0.0329 | 0.0045 (0.288; 0.093) | 3.14 (0.2019) | 1.156 | 0.0430 | 1.00 | null supported |
| c28B_m2 | 0.0171 | 0.0107 (0.033; 0.089) | 3.30 (0.2679) | 1.113 | 0.0244 | 1.00 | null supported |
| c28B_m3 | 0.0330 | 0.0062 (0.113; 0.085) | 3.24 (0.2859) | 1.140 | 0.0445 | 1.00 | null supported |
| c32A_m2 | 0.0010 | -0.0076 (0.819; 0.080) | 3.64 (0.0360) | 1.109 | 0.0261 | 1.00 | null supported |
| c32A_m3 | 0.0330 | 0.0117 (0.079; 0.102) | 4.15 (0.0005) | 1.126 | 0.0230 | 1.00 | curve effect suspected → follow-up 1: not replicated |
| c32B_m2 | 0.0010 | -0.0029 (0.710; 0.075) | 2.83 (0.7616) | 1.095 | 0.0220 | 1.00 | null supported |
| c32B_m3 | 0.0329 | -0.0009 (0.528; 0.078) | 3.24 (0.2989) | 1.117 | 0.0278 | 1.00 | null supported |

**Follow-up 1 (FOLLOWUP_c32A_m3.md, note F1.1, preregistered before data).** Fresh seed
363589115, curve 80 of c32A against 15 comparators chosen by a fixed rule, plus 8 u-scaled
isomorphic models of curve 80, T = 12 targets, 2880 runs, audit OK (12/12 replays).

| test | ratio (95% CI) | p |
|---|---|---|
| T1 curve 80 vs comparators | 1.0021 (0.9815–1.0232) | 0.84 |
| T2 iso models vs comparators | 1.0009 (0.9921–1.0098) | 0.84 |
| T3 curve 80 vs its iso models | 1.0012 | 0.91 |
| positive control (planted +20% arm vs comparators) | 1.1906 | 6e-71 (met) |
| negative control (comparator leave-one-out) | D = 1.298, max abs z 2.55 | informative |

Decision (section 8, step 2): **not replicated — chance (winner's curse)**. The original c32A
m=3 flag is a false positive.

**Reading.** In 9 of 12 cells the null is supported with planted-effect power ≥ 0.99; the
bootstrap upper bound on the curve share of yield variance is ≤ 0.10 in every cell. Two m=3
cells at 24 bits are not informative at a 20% effect (power 0.38 and 0.84) and are not counted as
nulls. The one significant flag (c32A m=3, curve 80) did not replicate on fresh seeds. No curve of either class shows a
relation-yield effect beyond sampling noise. Every class member has the same group order, so the
predicted yield per attempt is identical across the class; the data agree. The analogue classes
have small h (|t| near the Hasse bound), unlike ECCp-131's h ≈ 2^63.6 (PROTOCOL.md limits).

Recorded deviation: the executed specs used R = 200 relations per run (spec generator default)
where PROTOCOL.md §3 says min(100, 0.6N); this raises the attempt count, the analysis is unchanged.
