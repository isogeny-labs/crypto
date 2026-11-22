#set page(paper: "a4", margin: 1.6cm)
#set text(font: "Libertinus Serif", size: 9.5pt)
#set par(justify: true)
#show heading: set text(size: 12pt)
#set table(stroke: 0.4pt + luma(160), inset: 3pt)
#align(center)[#text(size: 14pt, weight: "bold")[Reduced factor base, two large primes and sparse LA on genus-2 Jacobians over F\_(q^2): local development check]]
#align(center)[Status: \[pcc:unverified\]. Slug `extfield-jacobian-decomp`, round 2, 2026-11-21.]

= Question

Round 1 showed that Gaudry/Nagao decomposition index calculus on the genus-2 Jacobian J(F\_(q^2)) with the full x-in-F\_q factor base, no large primes and dense linear algebra has exponent 2 in q, the same as Pollard rho, and loses to matched rho once the solver is priced. This round builds the lever round 1 lacked: a reduced factor base of about c q^alpha classes, partial relations with up to two points outside it (large primes) merged by the repository's large-prime graph filter, and sparse linear algebra, so that relation phase (heuristically q^(3-2 alpha)) and linear algebra (q^(2 alpha)) balance below exponent 2 (1.5 at alpha = 0.75). Question: does this lower the counted cost, or its exponent, at q \<= 2039, and how far is it from matched negation-map rho at 0, 0.1 and 1 GAE per unpriced native operation? Local development check at small parameters; every number is \[pcc:unverified\]; no improvement is claimed.

= What changed since the first round

- Factor base: F = the min(n\_classes, round(c q^alpha)) classes with the smallest hash of x (nested across arms); alpha = 1 is the control (all classes, no large primes). Arms: (alpha, c) in {(1, 1), (0.75, 1), (0.75, 2), (0.6, 1), (0.6, 2)}. A verified decomposition is accepted with at most two classes outside F. \[pcc:unverified\]
- Merging: `filter_large_primes` of `src/cryptanalysis/large_prime_filter.rs` (hypergraph peeling plus Markowitz elimination on the two-core), copied unchanged into this directory with counting shims for its hooks. Every filter field operation and data word is priced.
- Sparse LA: sequential Wiedemann mod ell on the merged rows (normal equations, Berlekamp-Massey, residual check), every mulmod counted; round 1 used dense elimination. The recovered k is accepted only if \[k\]D = Q.
- Paired design: one decomposition stream per instance feeds all five arms (same R walk, same solver calls, same verification); each arm stops at its own first solving attempt and is charged the stream counters at that trial plus its own filter, LA and kD-check work.
- Same instances and seeds as round 1 (curve generation unchanged); rho re-run on those instances with the unchanged round-1 code. Control stream counts agree with round 1 (e.g. q = 2039 control S(0.1) mean 1.15e9 here, 1.13e9 in round 1).
- Ladder q in {131, 251, 509, 1021, 2039} (five points); q = 4093 not attempted: the round-1 solver's lazy u32 elimination stops at q = 2039 and decomp.rs was not edited.

= Method

- Instance: genus-2 curve y^2 = f(x) over F\_(q^2), random monic quintic f, \#J = h ell with ell prime, h \<= 16 (not charged), D of order ell, Q = k D. Same instances for IC arms and rho.
- Relation: R = cD + dQ walked through a 16-entry pool; the exact Weil-restriction solver (four quadrics, Macaulay degree 5 to 7) returns all decompositions R + P1 + .. + P4 = 0 with x(P\_i) in F\_q; each is verified in the group \[pcc:unverified\]. Row: sum over F-classes coef phi + d k = -c, plus at most two large-prime columns.
- Counted unit: group additions (GAE). Native work priced at w in {0, 0.1, 1} per operation (w\_cal from round 1's calibration is in RESULTS.json): solver F\_q multiply-adds (round 1's nominal dense Macaulay count), root finding, factor-base construction, filter operations (an inversion counts 40), Wiedemann mulmods. S = group additions + w x native. Rho's native tally is round 1's, unchanged.
- Seeds: instance seeds 1, 2, 3; stream tag 0; A/A is tag 1 on seed 1 at q in {131, 251, 509}. Exponents: least squares of ln S on ln q over the five cell means, bootstrap over the 3 seeds per q (2000 resamples). Ratios S(arm)/S(rho) are paired by instance with a bootstrap over seeds.
- Stopping rule from `PLAN.md`: 1.5e6 trials or 20 minutes per stream; no arm-run was censored.

= Results

Distance to matched rho, S(arm)/S(rho) at w = 0, paired, 95% seed-bootstrap interval (below 1 means cheaper than rho) \[pcc:unverified\]:

#block(breakable: false)[
#set text(size: 6.5pt)
#table(columns: 6, align: left, [*q*], [*control\_a1*], [*a0.75\_c1*], [*a0.75\_c2*], [*a0.6\_c1*], [*a0.6\_c2*], [131], [0.205 \[0.148, 0.308\]], [0.22 \[0.164, 0.327\]], [0.205 \[0.148, 0.308\]], [0.41 \[0.253, 0.667\]], [0.227 \[0.165, 0.353\]], [251], [0.121 \[0.0912, 0.208\]], [0.157 \[0.112, 0.28\]], [0.12 \[0.0912, 0.203\]], [0.397 \[0.255, 0.717\]], [0.177 \[0.125, 0.301\]], [509], [0.105 \[0.0654, 0.301\]], [0.169 \[0.106, 0.472\]], [0.104 \[0.065, 0.292\]], [0.61 \[0.394, 1.7\]], [0.217 \[0.142, 0.576\]], [1021], [0.0397 \[0.0361, 0.0481\]], [0.0855 \[0.0768, 0.0986\]], [0.0438 \[0.0383, 0.0544\]], [0.374 \[0.323, 0.467\]], [0.126 \[0.113, 0.15\]], [2039], [0.0381 \[0.029, 0.0548\]], [0.1 \[0.0752, 0.143\]], [0.0469 \[0.0363, 0.0699\]], [0.623 \[0.516, 0.861\]], [0.193 \[0.15, 0.283\]])
]

Distance to matched rho, S(arm)/S(rho) at w = 0.1, paired, 95% seed-bootstrap interval (below 1 means cheaper than rho) \[pcc:unverified\]:

#block(breakable: false)[
#set text(size: 6.5pt)
#table(columns: 6, align: left, [*q*], [*control\_a1*], [*a0.75\_c1*], [*a0.75\_c2*], [*a0.6\_c1*], [*a0.6\_c2*], [131], [4.66e+03 \[3.09e+03, 7.4e+03\]], [5.17e+03 \[3.64e+03, 8.19e+03\]], [4.66e+03 \[3.09e+03, 7.4e+03\]], [1.18e+04 \[6.44e+03, 2.05e+04\]], [5.39e+03 \[3.65e+03, 9.06e+03\]], [251], [2.87e+03 \[2.22e+03, 4.51e+03\]], [4e+03 \[2.88e+03, 6.74e+03\]], [2.84e+03 \[2.22e+03, 4.32e+03\]], [1.16e+04 \[7.38e+03, 2.07e+04\]], [4.68e+03 \[3.33e+03, 7.4e+03\]], [509], [2.55e+03 \[1.52e+03, 7.97e+03\]], [4.38e+03 \[2.67e+03, 1.33e+04\]], [2.52e+03 \[1.51e+03, 7.67e+03\]], [1.72e+04 \[1.08e+04, 5.15e+04\]], [5.77e+03 \[3.67e+03, 1.66e+04\]], [1021], [1.01e+03 \[912, 1.22e+03\]], [2.31e+03 \[2.13e+03, 2.67e+03\]], [1.13e+03 \[1.01e+03, 1.4e+03\]], [1.05e+04 \[9.22e+03, 1.31e+04\]], [3.45e+03 \[3.17e+03, 4.13e+03\]], [2039], [1.02e+03 \[763, 1.47e+03\]], [2.77e+03 \[2.06e+03, 3.98e+03\]], [1.26e+03 \[965, 1.9e+03\]], [1.75e+04 \[1.44e+04, 2.43e+04\]], [5.4e+03 \[4.17e+03, 7.92e+03\]])
]

Distance to matched rho, S(arm)/S(rho) at w = 1, paired, 95% seed-bootstrap interval (below 1 means cheaper than rho) \[pcc:unverified\]:

#block(breakable: false)[
#set text(size: 6.5pt)
#table(columns: 6, align: left, [*q*], [*control\_a1*], [*a0.75\_c1*], [*a0.75\_c2*], [*a0.6\_c1*], [*a0.6\_c2*], [131], [1.88e+04 \[1.14e+04, 3.96e+04\]], [2.09e+04 \[1.34e+04, 4.38e+04\]], [1.88e+04 \[1.14e+04, 3.96e+04\]], [4.75e+04 \[2.38e+04, 1.1e+05\]], [2.18e+04 \[1.35e+04, 4.84e+04\]], [251], [9.52e+03 \[7.29e+03, 1.68e+04\]], [1.33e+04 \[9.45e+03, 2.51e+04\]], [9.43e+03 \[7.29e+03, 1.61e+04\]], [3.86e+04 \[2.42e+04, 7.69e+04\]], [1.55e+04 \[1.09e+04, 2.76e+04\]], [509], [7.68e+03 \[4.48e+03, 2.59e+04\]], [1.32e+04 \[7.84e+03, 4.32e+04\]], [7.57e+03 \[4.46e+03, 2.49e+04\]], [5.17e+04 \[3.17e+04, 1.67e+05\]], [1.74e+04 \[1.08e+04, 5.37e+04\]], [1021], [2.88e+03 \[2.6e+03, 3.47e+03\]], [6.6e+03 \[6.07e+03, 7.63e+03\]], [3.21e+03 \[2.89e+03, 4e+03\]], [2.98e+04 \[2.63e+04, 3.74e+04\]], [9.83e+03 \[9.04e+03, 1.18e+04\]], [2039], [2.88e+03 \[2.16e+03, 4.17e+03\]], [7.84e+03 \[5.82e+03, 1.13e+04\]], [3.58e+03 \[2.73e+03, 5.38e+03\]], [4.96e+04 \[4.07e+04, 6.9e+04\]], [1.53e+04 \[1.18e+04, 2.25e+04\]])
]

At q = 2039 the distance to rho at w = 0, 0.1 and 1 is, per arm: control\_a1 0.0381, 1.02e+03, 2.88e+03; a0.75\_c1 0.1, 2.77e+03, 7.84e+03; a0.75\_c2 0.0469, 1.26e+03, 3.58e+03; a0.6\_c1 0.623, 1.75e+04, 4.96e+04; a0.6\_c2 0.193, 5.4e+03, 1.53e+04. The best arm at every priced w is the control; no reduced-factor-base arm is closer to rho than the control at q = 2039. \[pcc:unverified\]

Mean total S at w = 0.1 (GAE) \[pcc:unverified\]:

#block(breakable: false)[
#set text(size: 6.5pt)
#table(columns: 7, align: left, [*q*], [*rho*], [*control\_a1*], [*a0.75\_c1*], [*a0.75\_c2*], [*a0.6\_c1*], [*a0.6\_c2*], [131], [2e+04], [9.35e+07], [1.04e+08], [9.35e+07], [2.36e+08], [1.08e+08], [251], [5.67e+04], [1.63e+08], [2.27e+08], [1.61e+08], [6.6e+08], [2.65e+08], [509], [1.15e+05], [2.94e+08], [5.05e+08], [2.9e+08], [1.98e+09], [6.65e+08], [1021], [5.8e+05], [5.86e+08], [1.34e+09], [6.53e+08], [6.07e+09], [2e+09], [2039], [1.13e+06], [1.15e+09], [3.13e+09], [1.43e+09], [1.98e+10], [6.1e+09])
]

Fitted exponents b in S = c q^b over the five cell means, 95% bootstrap interval \[pcc:unverified\]:

#block(breakable: false)[
#set text(size: 8pt)
#table(columns: 5, align: left, [*arm*], [*w = 0*], [*w = 0.1*], [*w = 1*], [*trials to solve*], [control\_a1], [0.80 \[0.764, 0.844\]], [0.91 \[0.853, 0.975\]], [0.91 \[0.856, 0.976\]], [0.93 \[0.87, 0.998\]], [a0.75\_c1], [1.14 \[1.1, 1.17\]], [1.25 \[1.2, 1.3\]], [1.25 \[1.2, 1.3\]], [1.27 \[1.21, 1.32\]], [a0.75\_c2], [0.88 \[0.841, 0.92\]], [0.99 \[0.939, 1.06\]], [0.99 \[0.94, 1.06\]], [1.01 \[0.948, 1.08\]], [a0.6\_c1], [1.57 \[1.51, 1.63\]], [1.61 \[1.54, 1.69\]], [1.61 \[1.54, 1.69\]], [1.63 \[1.55, 1.72\]], [a0.6\_c2], [1.36 \[1.32, 1.39\]], [1.46 \[1.42, 1.52\]], [1.46 \[1.41, 1.52\]], [1.48 \[1.43, 1.54\]], [rho], [1.45 \[1.31, 1.61\]], [1.51 \[1.38, 1.68\]], [1.63 \[1.47, 1.86\]], [n/a])
]

Cell detail (means over 3 seeds): factor-base size nF (classes), accepted rows (partial plus complete), merged complete rows and columns at the solving attempt, and the LA share of S(0.1) \[pcc:unverified\]:

#block[
#set text(size: 6.5pt)
#table(columns: 9, align: left, [*arm*], [*q*], [*nF*], [*f*], [*trials*], [*rows*], [*merged*], [*cols*], [*LA share*], [control\_a1], [131], [60], [1.00], [1.94e+03], [76.3], [76.3], [66.3], [2.0e-04], [control\_a1], [251], [125], [1.00], [3.45e+03], [134], [134], [124], [3.6e-04], [control\_a1], [509], [261], [1.00], [6.41e+03], [266], [266], [252], [8.0e-04], [control\_a1], [1021], [505], [1.00], [1.28e+04], [516], [516], [500], [1.5e-03], [control\_a1], [2039], [1032], [1.00], [2.49e+04], [1.03e+03], [1.03e+03], [1.01e+03], [3.2e-03], [a0.75\_c1], [131], [39], [0.65], [2.15e+03], [72.3], [47], [39.7], [1.1e-04], [a0.75\_c1], [251], [63], [0.50], [4.8e+03], [128], [72.3], [64], [1.3e-04], [a0.75\_c1], [509], [107], [0.41], [1.1e+04], [261], [124], [107], [2.0e-04], [a0.75\_c1], [1021], [181], [0.36], [2.95e+04], [506], [206], [181], [2.2e-04], [a0.75\_c1], [2039], [303], [0.29], [6.8e+04], [973], [329], [303], [2.7e-04], [a0.75\_c2], [131], [60\*], [1.00], [1.94e+03], [76.3], [76.3], [66.3], [2.0e-04], [a0.75\_c2], [251], [125\*], [1.00], [3.42e+03], [132], [130], [122], [3.5e-04], [a0.75\_c2], [509], [214], [0.82], [6.31e+03], [262], [221], [211], [6.8e-04], [a0.75\_c2], [1021], [361], [0.71], [1.43e+04], [525], [382], [357], [9.9e-04], [a0.75\_c2], [2039], [607], [0.59], [3.09e+04], [1.06e+03], [658], [603], [1.5e-03], [a0.6\_c1], [131], [19], [0.32], [4.81e+03], [72.3], [27.7], [20], [2.7e-05], [a0.6\_c1], [251], [28], [0.22], [1.4e+04], [118], [33.7], [29], [1.9e-05], [a0.6\_c1], [509], [42], [0.16], [4.33e+04], [250], [58.7], [43], [1.9e-05], [a0.6\_c1], [1021], [64], [0.13], [1.33e+05], [446], [81.7], [65], [1.4e-05], [a0.6\_c1], [2039], [97], [0.09], [4.31e+05], [864], [117], [97.7], [1.1e-05], [a0.6\_c2], [131], [37], [0.62], [2.24e+03], [74], [46.7], [37.7], [1.1e-04], [a0.6\_c2], [251], [55], [0.44], [5.6e+03], [122], [59], [55], [9.4e-05], [a0.6\_c2], [509], [84], [0.32], [1.45e+04], [254], [97.7], [83.7], [1.1e-04], [a0.6\_c2], [1021], [128], [0.25], [4.39e+04], [468], [140], [129], [9.1e-05], [a0.6\_c2], [2039], [193], [0.19], [1.33e+05], [919], [223], [194], [8.4e-05])
]

An asterisk on nF marks an arm clamped to the whole factor base (identical to the control at that q).

= Results against the preregistered predictions

#block(breakable: false)[
#table(columns: 3, align: left, [*id*], [*outcome*], [*evidence*], [P1], [alpha=0.75 part confirmed; control part refuted], [exponents (w0, w0.1, w1): a0.75\_c1 1.14, 1.25, 1.25; a0.75\_c2 0.88, 0.99, 0.99; control 0.80, 0.91, 0.91 (assigned control band 1.9 to 2.1). Own expectation: alpha=0.75 about 1.3 to 1.6 (measured below it), control 0.8 to 1.2 (matches).], [P2], [confirmed], [max LA share of S at q=2039: 0.0035 at w=0.1 and w=1, 0.002 at w=0 (any arm, any seed)], [P3], [refuted], [best arm at q=2039 is control\_a1; S/rho at w=0.1 along q: 4.66e+03, 2.87e+03, 2.55e+03, 1.01e+03, 1.02e+03], [P4], [confirmed qualitatively], [S/rho at q=2039, w=0.1: control\_a1 1.02e+03, a0.75\_c1 2.77e+03, a0.75\_c2 1.26e+03, a0.6\_c1 1.75e+04, a0.6\_c2 5.4e+03; every projected q\* is far beyond the ladder (see the projection table)], [P5], [ordering confirmed at the top q; second clause refuted narrowly], [ordering at q=2039 holds; arm/control below 1 at: a0.75\_c2 q=251 (0.990), a0.75\_c2 q=509 (0.986)], [P6], [confirmed], [solver native per trial q=131: 4.91e+05, q=251: 4.71e+05, q=509: 4.55e+05, q=1021: 4.51e+05, q=2039: 4.51e+05; min native share of S(0.1) at q\>=509: 0.9968], [P7], [confirmed], [measured/model acceptance rate over 12 cells: min 0.92, max 1.16 (band 0.7 to 1.3)], [P8], [confirmed], [A/A S ratios (tag 0 over tag 1, seed 1, q in 131, 251, 509, all arms): min 0.98, max 1.34], [P9], [confirmed], [0 censored and 0 unsolved arm-runs of 75; attempts that returned a wrong k: 0; unit tests pass (12 tests, incl. filter plus Wiedemann on synthetic relations)], [P10], [confirmed], [raw rows at solve over model: min 0.76, max 0.98 over 12 cells (factor-2 band)])
]

- Per-trial decomposition cost dominates everything: the solver costs about 4.51e+05 counted F\_q mulmods and 0.52 ms of wall time per trial, independent of q and of the arm. At w = 0.1 that is about 4.5e+04 GAE per trial, so the native share of S(0.1) is at least 0.997 in every arm and cell at q \>= 509, and the LA share is at most 3.5e-03 at q = 2039 (at w = 0, where only group additions count, at most 2.0e-03). The measured number of rows needed is about 0.5 q for every arm (the large-prime graph needs about one relation per large-prime vertex), so a reduced factor base only multiplies the number of trials: trials to solve grow like q^1.63 for alpha = 0.6 (theory 1.8), q^1.27 for alpha = 0.75 (theory 1.5), q^0.93 for the control (theory 1).
- The assigned P1 holds for alpha = 0.75 (exponents below 1.7) but it does not indicate an improvement \[pcc:unverified\]: the control's own ladder exponent is 0.91 at w = 0.1 and every reduced arm's is higher than the control's except the clamped (0.75, 2) arm, which equals the control at q \<= 251. The ladder is solver-dominated; the LA term that the reduced factor base shrinks is below 3e-03 of S, so shrinking it cannot pay at q \<= 2039.
- Honest crossover projection (extrapolation, two-term theory model anchored at the q = 2039 means, rho \~ 0.44 q^2 at w = 0.1 and 1.48 q^2 at w = 1): at w = 0.1 the first q where an arm drops below rho is 1.23e+09 for (0.75, 2) and 5.88e+09 for (0.75, 1), none for the control (LA exponent 2) and none below 1e23 for (0.6, 1); at w = 1 it is 1.11e+10 and 5.3e+10. Group order is about q^4, so these are orders of 1e36 to 1e43: far outside any measurement. At w = 0 every arm is already below rho on the whole ladder (0.04 to 0.6 of rho at q = 2039), which only says mulmods are free there.

= Limitations

- Five q values from 131 to 2039 and three seeds per cell: exponents are ladder-local, not asymptotic. At small q the c = 2 arms are clamped to the whole factor base and alpha = 0.75 has f above 0.4, so reduced-base behaviour is only visible at the top of the ladder.
- The projection table extrapolates over about eight orders of magnitude from two exponents taken as theory; it is not a measurement. Rho is modelled as c q^2 from the q \>= 509 cells (round 1's measured rho exponent is biased low by a fixed overhead).
- Counting conventions: Macaulay elimination is round 1's nominal dense count (an upper bound; a sparse F4 would charge less), filter data words are priced like field operations, an inversion is 40 mulmods, Wiedemann is the sequential variant on normal equations (not block Wiedemann). The solver's per-trial cost is therefore the least certain constant; w\_cal (shared 4-CPU VM) is not used for the headline rows.
- The large-prime filter is a copy of the repository module (tests and `trace` submodule dropped, hooks replaced by counters); its fill limits were left at the defaults (4096); the number of fill rejections was not recorded.
- The relation walk and its k column follow round 1; points with y = 0 carry no column. Large primes are classes with x in F\_q only (no points with x outside F\_q).
- `cargo clippy --release --bin extfield_jac_decomp -- -D warnings` fails on 13 pre-existing errors in library modules this task may not edit (`prime_radical_root.rs`, `prime_root_trace.rs` and others; 13 errors in this build); the bin and the files of this directory produce no clippy warnings. `cargo fmt` is clean for them.
- Local development check: no isolab, benchrec, replay certificate or validator report; the research-kb knowledge base was not queried in this run. Every number is \[pcc:unverified\] and must not enter a leaderboard row.

= Reproduction

From /home/user/crypto at git HEAD 473c4064294aea5131fb869b8aaa30810f9ed228 (working tree with the new files). Counts are deterministic given the seeds (a re-run of q = 131 seed 1 reproduced every count); wall times vary.

#block(fill: luma(240), inset: 5pt, width: 100%)[#text(size: 7.5pt)[```
CARGO_TARGET_DIR=/home/user/crypto/target cargo build --release --bin extfield_jac_decomp -j2 && ./research/extfield-jacobian-decomp_20261121/run_ladder.sh && python3 research/extfield-jacobian-decomp_20261121/analyze.py && python3 research/extfield-jacobian-decomp_20261121/graph.py && python3 research/extfield-jacobian-decomp_20261121/build_report.py
```]]

Smallest cell: `GIT_HEAD=$(git log -1 --format=%H) ./target/release/extfield_jac_decomp lp 131 1 0` (about 4 s). Unit tests: `CARGO_TARGET_DIR=/home/user/crypto/target cargo test --release --bin extfield_jac_decomp`.

= Sources read

- `research/extfield-jacobian-decomp_20261115/` (PLAN.md, REPORT.md, RESULTS.json, src/decomp.rs, src/jac.rs, src/fq2.rs, run\_ladder.sh) and `src/bin/extfield_jac_decomp.rs`.
- `src/cryptanalysis/large_prime_filter.rs` (read in full, copied), `src/cryptanalysis/jv_bitops.rs` (hook names), `src/cryptanalysis/pq_wiedemann.rs` (header and wiedemann\_solve; not used: BigUint and crate-internal rows), `src/cryptanalysis/jv_sieve_two_large_prime.rs` (how partial rows are built for the filter).
- Function lists only: `src/cryptanalysis/diem_quartic.rs` (eliminate\_lp), `research/quartic_n4_la_20261021/ic.rs`. `CLAUDE.md` rules, `docs/ic/SUCCESS_CRITERIA.md` not opened.

#figure(image("graph.svg", width: 100%), caption: [Total S at w = 0.1 (left) and distance to matched rho (right) versus q, all arms. \[pcc:unverified\]])
