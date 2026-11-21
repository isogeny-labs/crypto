# REPORT: genus-4 (3,3) curves, round 2 (root-finding relation phase, large primes, preregistered factor-base rule)

All numbers are [pcc:unverified]. Local development check at small parameters, not a lab benchmark; no improvement is claimed. git HEAD at run time `473c4064294aea5131fb869b8aaa30810f9ed228` (working tree uncommitted). Plan: `PLAN.md` (frozen before the first measurement run).

## Question

Round 1 closed discrete logarithms in Pic^0 of smooth (3,3) curves on P1 x P1 over F_p (genus 4, non-hyperelliptic) with relations from trigonal fibres and plane-quintic lines, but its relation phase scanned the whole factor base per pair (native cost about m^3, p^2.2 measured), its headline used an oracle rung, and its ladder (p up to 509) was too short for an exponent. Does an exact root-finding lookup plus a large-prime step make the relation phase scale, and what is the distance to a matched rho on the same group at 0, 0.1 and 1 GAE per native operation?

## What changed since the first round

- Relation phase: the residual of a plane section is a binary cubic over F_p; its roots come from a deterministic root finder and a hash lookup into F (per pair about 800 to 1400 counted F_p multiplications, growing slowly with log p, independent of m). The scan remains only for degenerate sections (about 1/p of pairs).
- Large primes: A- and B-relations with one or two residual points outside F go through `large_prime_filter::filter_large_primes` (library module, unchanged). The solve also gives one-step logs of large-prime points, which enlarge the decomposition base U used by the individual-log step.
- Factor base: sampled by root finding of random fibres, sized by the preregistered rule m = ceil(c0 * 1.25^rung * p^alpha) with alpha in {0.5, 2/3, 0.75}; the primary cost is the sum over ALL rungs (the final-rung cost is kept in `RESULTS.json` as `S_fin_*`).
- Individual log: one scale record per curve plus one record per target (3 targets per curve), decomposing over U with a 9-multiplication test per U point per step instead of the round-1 scan of all points.
- Rho: plain DP rho, same arithmetic and group. The negation map is not built: negation costs one full combine here (`neg = comb(0,0,x)`), so plain rho is used in both arms. The enumeration of all points (a p^2 charge in round 1) is no longer charged to IC; instance work (point counting, group order, fibre statistics) is uncharged for both arms.

## Method

Cells: p in {251, 509, 1021, 2039}, 3 curves per p (curve seeds in `RESULTS.json`), 3 DLP targets per curve, each arm's ladder run twice (A/A). Rho: one run per target at p <= 1021 (up to 9 per p), one run on target 0 per curve at p = 2039. Cost unit GAE = one combine (counted exactly); native work = F_p multiplications plus mod-ell word operations (Wiedemann and the filter's own `jv_bitops` count), priced at w = 0, 0.1, 1 GAE per operation and at the calibrated w_cal = t(mulmod)/t(add) of the same binary (about 6e-6). Per-target IC cost = all rungs' shared cost + that target's own cost. Exponents: log-log OLS over the 4 ladder points, bootstrap 95% interval over curves (runs for rho). Repro: `bash repro.sh` (build, tests, yield study, ladder, analysis, figure, PDF).

## Results against the preregistered predictions

| id | prediction | measured | verdict |
|---|---|---|---|
| R1 | relation-phase native exponent <= 1.4 | alpha=0.500: all rungs 1.46 [1.45, 1.48], final rung 1.46 [1.45, 1.48]; alpha=0.667: all rungs 1.30 [1.26, 1.34], final rung 1.33 [1.29, 1.36]; alpha=0.750: all rungs 1.18 [1.17, 1.19], final rung 1.33 [1.32, 1.34] | alpha=0.500 refuted, alpha=0.667 holds, alpha=0.750 holds |
| R2 | total-S exponent <= 1.6 (rho 2.0 +- 0.15) | alpha=0.500: w=0 0.03 [-0.02, 0.07], w=0.1 1.49 [1.47, 1.51], w=1 1.49 [1.47, 1.51]; alpha=0.667: w=0 0.09 [0.03, 0.16], w=0.1 1.32 [1.28, 1.36], w=1 1.32 [1.28, 1.36]; alpha=0.750: w=0 0.01 [-0.03, 0.07], w=0.1 1.21 [1.19, 1.23], w=1 1.21 [1.19, 1.23]; rho adds 2.04 [1.75, 2.33] | alpha=0.500 holds, alpha=0.667 holds, alpha=0.750 holds; rho inside 2.0 +- 0.15 |
| R3 | S_IC/S_rho at w=0.1 decreasing in p | alpha=0.500: 22.3, 18.6, 16, 6.4; alpha=0.667: 24.1, 20.2, 10.6, 5.52; alpha=0.750: 22.7, 15.3, 12.1, 3.49 | alpha=0.500 monotone, alpha=0.667 monotone, alpha=0.750 monotone |
| R4 | 1/6 fibre law within +-25% | pooled per p: 251: 0.169, 509: 0.167, 1021: 0.17, 2039: 0.167 | holds |
| R5 | A-line yield = phi^3/6 within +-25% (>= 100 hits); scan == root finder at p <= 509 | ratios 251/phi=0.5: 1.07, 251/phi=1.0: 1.06, 509/phi=0.5: 0.991, 509/phi=1.0: 1, 1021/phi=0.5: 0.982, 1021/phi=1.0: 1.03, 2039/phi=0.5: 1.02, 2039/phi=1.0: 1.02; scan equality 8 of 8 cells | holds |
| R6 | at alpha=2/3 the solved rung has phi below the A+B-only phi* of round 1 (0.8-1 at p = 251-509) | alpha=0.500 phi at solved rung: 251: 0.43, 509: 0.29, 1021: 0.27, 2039: 0.19; alpha=0.667 phi at solved rung: 251: 0.46, 509: 0.31, 1021: 0.24, 2039: 0.19; alpha=0.750 phi at solved rung: 251: 0.46, 509: 0.3, 1021: 0.27, 2039: 0.18 | holds at every p (alpha = 2/3: phi 0.46 to 0.19, round 1 needed 0.8 to 1.0); caveat: round 1 used A+B only and no large primes |
| R7 | every target of a solved rung verified; rho verified | IC passes solved: 72 of 72 (rows listed in RESULTS.md); rho verified 30 of 30 | holds |
| R8 | A/A within +-30% in GAE (w=0) | alpha=0.500 pass1/pass0 S(w=0): median 0.998 range 0.866..1.29; alpha=0.667 pass1/pass0 S(w=0): median 0.969 range 0.786..1.29; alpha=0.750 pass1/pass0 S(w=0): median 1.05 range 0.77..1.19 | holds: all pairs within 0.77..1.29, and both passes solve at the same rung |
| R9 | crossover projection | see next section | - |

## Distance to matched rho (all rungs, per target, pooled measured rho)

| alpha | p | S_IC median w=0 | S_IC median w=0.1 | S_IC median w=1 | rho adds | rho adds + 0.1 native | rho adds + 1 native | ratio w=0 | ratio w=0.1 | ratio w=1 | ratio w_cal |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 0.500 | 251 | 350 | 1.74e+06 | 1.74e+07 | 7.11e+04 | 7.81e+04 | 1.42e+05 | 0.00493 | 22.3 | 123 | 0.00643 |
| 0.500 | 509 | 394 | 4.42e+06 | 4.42e+07 | 2.17e+05 | 2.38e+05 | 4.33e+05 | 0.00182 | 18.6 | 102 | 0.00309 |
| 0.500 | 1021 | 373 | 1.62e+07 | 1.62e+08 | 9.23e+05 | 1.02e+06 | 1.85e+06 | 0.000404 | 16 | 87.9 | 0.00141 |
| 0.500 | 2039 | 379 | 3.59e+07 | 3.59e+08 | 5.1e+06 | 5.61e+06 | 1.02e+07 | 7.43e-05 | 6.4 | 35.2 | 0.000505 |
| 0.667 | 251 | 359 | 1.88e+06 | 1.88e+07 | 7.11e+04 | 7.81e+04 | 1.42e+05 | 0.00505 | 24.1 | 133 | 0.00669 |
| 0.667 | 509 | 408 | 4.82e+06 | 4.82e+07 | 2.17e+05 | 2.38e+05 | 4.33e+05 | 0.00188 | 20.2 | 111 | 0.0032 |
| 0.667 | 1021 | 409 | 1.07e+07 | 1.07e+08 | 9.23e+05 | 1.02e+06 | 1.85e+06 | 0.000443 | 10.6 | 58.2 | 0.0011 |
| 0.667 | 2039 | 438 | 3.09e+07 | 3.09e+08 | 5.1e+06 | 5.61e+06 | 1.02e+07 | 8.6e-05 | 5.52 | 30.3 | 0.000464 |
| 0.750 | 251 | 358 | 1.77e+06 | 1.77e+07 | 7.11e+04 | 7.81e+04 | 1.42e+05 | 0.00503 | 22.7 | 125 | 0.00656 |
| 0.750 | 509 | 373 | 3.64e+06 | 3.64e+07 | 2.17e+05 | 2.38e+05 | 4.33e+05 | 0.00172 | 15.3 | 84.1 | 0.00277 |
| 0.750 | 1021 | 365 | 1.23e+07 | 1.23e+08 | 9.23e+05 | 1.02e+06 | 1.85e+06 | 0.000396 | 12.1 | 66.5 | 0.00118 |
| 0.750 | 2039 | 373 | 1.96e+07 | 1.96e+08 | 5.1e+06 | 5.61e+06 | 1.02e+07 | 7.31e-05 | 3.49 | 19.2 | 0.000313 |

## Crossover projection (R9)

Projection assumes the fitted exponents continue: S_IC/S_rho = ratio(2039) * (p/2039)^(e_IC - e_rho), crossing 1 at p* = 2039 * ratio(2039)^(1/(e_rho - e_IC)), with e_rho = 2 (theory) and with the measured rho exponent. Extrapolation of a 4-point fit by one to several orders of magnitude; the point is distance, not a forecast.

| alpha | w | ratio at 2039 | e_IC | p* with e_rho = 2 | p* with measured e_rho |
|--:|--:|--:|--:|--:|--:|
| 0.500 | 0 | 7.43e-05 | 0.03 | already below 1 at p = 2039 | already below 1 at p = 2039 |
| 0.500 | 0.1 | 6.4 | 1.49 | 7.57e+04 | 5.73e+04 |
| 0.500 | 1 | 35.2 | 1.49 | 2.1e+06 | 1.23e+06 |
| 0.667 | 0 | 8.6e-05 | 0.09 | already below 1 at p = 2039 | already below 1 at p = 2039 |
| 0.667 | 0.1 | 5.52 | 1.32 | 2.49e+04 | 2.15e+04 |
| 0.667 | 1 | 30.3 | 1.32 | 3.04e+05 | 2.26e+05 |
| 0.750 | 0 | 7.31e-05 | 0.01 | already below 1 at p = 2039 | already below 1 at p = 2039 |
| 0.750 | 0.1 | 3.49 | 1.21 | 9.85e+03 | 9.09e+03 |
| 0.750 | 1 | 19.2 | 1.21 | 8.45e+04 | 6.98e+04 |

[[FIGURE]]

## Interpretation and limitations

- The 'all-rungs' exponents depend on the ladder discretisation: m jumps by a factor 1.25 per rung and the solved rung index falls with p, so the relation-phase fits are not smooth power laws. The bootstrap interval reflects curve-to-curve spread only (curves at one p solve at the same rung), so it understates the real uncertainty of the exponent. [pcc:unverified]
- The factor-base size needed for a solve grows much slower than p^0.75 here (m at the solved rung is in the table); with large primes the binding constraint is the large-prime graph, not the A+B count of round 1. At these sizes phi is still 0.2 to 0.5, so this is not yet the asymptotic regime of the heuristic.
- Heuristic check: I predicted m ~ p^alpha plus rungs and a relation exponent of 1.5 to 1.6 at alpha = 0.75; the measured needed m grows only like p^0.6 (fit on m at the solved rung, see RESULTS.json), so relation-phase work (about m^2 pairs) scales near p^1.2 to p^1.3 and the preregistered 1.4 holds for alpha >= 2/3, fails for alpha = 0.5 (1.46). The final-rung fits (1.33 to 1.46) are the less rung-dependent numbers.
- w = 0.1 and w = 1 give the same exponent because native work dominates both; S at w = 0 is the group-operation count only and is flat in p (exponent about 0 to 0.1), i.e. the ratio to rho at w = 0 falls like p^-2. That column is a bound under free native work, not a result about speed; no improvement is claimed.
- Rho at p = 2039 is 3 runs (adds 3.0e6, 3.5e6 and a third larger run, mean 5.1e6, relative error large); its exponent 2.04 has a wide interval [1.75, 2.33]. The ratio trend R3 and the crossover p* inherit that noise; the IC-side spread is small only because curves at one p solve at the same rung.
- At w = 0 IC costs a few hundred GAE (group operations are only the adders, scale record and per-target walks) and is flat in p; at w = 0.1 and w = 1 the native work (relation phase and Wiedemann) dominates and the ratio to rho is above 1 at every measured p. Under the calibrated w_cal (one combine = 2.7e5 field multiplications in this arithmetic) the ratio is far below 1; the 0.1 and 1 GAE prices are the repository's pessimistic band and rank the arms only where the order holds across it, which it does not.
- Rho is plain (no negation), 1 to 9 runs per p; a free-negation reference would lower rho by at most sqrt(2). Rho at p = 2039 has 2 or 3 runs; its mean has a relative standard error of about 0.5/sqrt(n).
- Decomposition scans U (9 multiplications per point per step), not a polylog support extraction; the individual-log native cost is included in all totals.
- Split quadric only; instance work uncharged; dense kernel check only up to 600 used columns, above that the verified final log is the check.
- Round-1 unit tests (in the round-1 binary) were not re-run this round; this round's 7 tests (root finder vs brute force, division, scan equality at p = 13..127, coordinate order, LP relations true in Pic at p = 31, sampled FB) pass.

## Sources read

- `research/low-gonality-genus4_20261115/{PLAN.md,REPORT.md,RESULTS.md,src/ic.rs,src/g4.rs}`, `src/bin/genus4_pencil_ic.rs`
- `src/cryptanalysis/large_prime_filter.rs`, `src/cryptanalysis/jv_bitops.rs`
