# PLAN (round 2): genus-4 (3,3) curves, root-finding relation phase, large primes, preregistered factor-base rule

Frozen 2026-11-21 before the first measurement run. Local development check at small parameters, not a lab benchmark.
Every number is `[pcc:unverified]`. No improvement is claimed. git HEAD at planning time: `473c4064294aea5131fb869b8aaa30810f9ed228` (working tree has
uncommitted files). Binary: `src/bin/genus4_pencil_ic_lp.rs` (this round), helpers `research/low-gonality-genus4_20261121/src/rf.rs`.
Round-1 code (`../low-gonality-genus4_20261115/src/{g4,ic}.rs`) is included unchanged for the Pic^0 arithmetic, Wiedemann and rho.

## What changes since round 1

1. Relation phase: the residual of the plane section through Pa, F[a], F[b] is a binary cubic over F_p (the plane section is a
   conic parametrised by (s:t); restricting G gives a sextic, three known roots are divided out). Its F_p roots come from a
   deterministic root finder (gcd with x^p - x, then gcd with (x+a)^((p-1)/2) - 1 for a = 0, 1, ...) and each root is looked up in
   a hash of F (cross-multiplication check inside the fibre). Per-pair cost is polylog in p, not m. Degenerate sections (about 1/p)
   fall back to a scan. Every field multiplication and hash probe is counted.
2. Large primes: A-relations with one or two residual points outside F, and B-relations (fibres of both rulings through an F point)
   with one or two points outside F, go through `large_prime_filter::filter_large_primes` (library module, unchanged). Complete rows
   over F plus kappa, tau1, tau2 feed Wiedemann as before. Large-prime logs known after the solve (one unknown per relation) extend
   the decomposition base U of the individual-log step.
3. Factor base: sampled by root finding of random fibres (uniform over points, accept with probability k/3), so no enumeration of the
   p^2 affine points is charged. Size rule m = ceil(c0 * 1.25^rung * p^alpha), arms (alpha, c0) = (0.5, 1.5), (2/3, 1.0), (0.75, 1.0),
   rungs 0..11 (RUNGS = 12; changed from 7 after a p = 251 smoke run showed alpha 0.5 and 2/3 ran out of rungs; no ladder data seen). alpha = 0.5 and 0.75 are the task's arms; 2/3 is added because the heuristic below puts the large-prime optimum there.
   Primary cost = ALL rungs (relation phase, LA and any individual-log work of every rung that was run); the final-rung cost is shown beside it.
4. Individual log: one record from a pure g-multiple walk fixes the unknown scale per curve, then ONE record per target
   (walk adders are multiples of g; start a g + b h). Decomposition: exactly four points of U vanish on W_D (cost 9 mulmods per U point per step).
   The old per-step scan of all rational points (45 p native) is gone.
5. Rho: plain DP rho, same arithmetic, same group. The negation map costs one full combine here (neg = comb(0,0,x)), so it is NOT built;
   plain rho in both arms. A free-negation reference would be plain/sqrt(2), labelled an assumption, not used in any ratio.

## Heuristics fixed now (so the predictions are not fitted afterwards)

phi = m/n, n ~ p. A-pairs m^2/2; a pair gives a split cubic with probability 1/6; the number of F points among the 3 residual points is
Binomial(3, phi). Complete A relations per pair phi^3/6. Relation-phase native work ~ 400 * m^2/2 mulmods (one gcd chain per pair), so
its exponent in p is 2 alpha (+ log). With one or two large primes the LP graph needs about m^3/(4p) ~ p/2 edges for cycles, so the
cheapest working alpha is about 2/3 (m ~ p^(2/3)); alpha = 0.5 should fail (too few relations) at the sizes reached. LA dimension ~ m
and cost ~ m^2 * weight. Individual-log steps ~ 24/phi_U^4 (phi_U = |U|/n).

## Preregistered predictions

| id | prediction | decision rule |
|---|---|---|
| R1 | relation-phase native-cost exponent in p (fit over the 4 ladder points, final rung and all rungs, per arm) <= 1.4 | exponent with bootstrap 95% interval; refuted if the point estimate > 1.4. My own expectation: alpha = 2/3 gives about 1.3-1.5, alpha = 0.75 about 1.5-1.6 (so I expect R1 to hold only for alpha <= 2/3), alpha = 0.5 undefined (fails) |
| R2 | total S exponent (w = 0, 0.1, 1 GAE, all rungs) <= 1.6 for the best arm; rho 2.0 +- 0.15 | bootstrap over curves; refuted if the point estimate > 1.6 |
| R3 | S_IC / S_rho at w = 0.1 decreasing in p over the 4 ladder points (per arm that completes), using pooled measured rho at each p | monotone decrease of the per-p median; any rise refutes |
| R4 | 1/6 fibre law holds at every p, both pencils, pooled over curves, within +-25% (0.125..0.208) | counted |
| R5 | A-line yield per pair phi^3/6 holds within +-25% at phi = 0.5 and 1.0 at every p (cells with >= 100 hits) and the root-finder count equals the scan count exactly at p <= 509 | exact equality for the scan check |
| R6 | large primes help: at the arm alpha = 2/3, the first solved rung has m smaller than the A+B-only phi* of round 1 (phi* 0.8-1 at p = 251-509) | m / n at the solved rung |
| R7 | DLP solved and verified (x g = h) for every target of every solved rung; rho verified in every run | hard check |
| R8 | A/A: the two passes of an arm agree in solved rung within +-1 rung and in total GAE (w = 0) within +-30% | counted |
| R9 | crossover: at the measured exponents IC/rho = const * p^(e_IC - 2); I will report the projected p where S_IC(w) = S_rho for w = 0, 0.1, 1 and say whether it is inside any realistic range | arithmetic from fits |

## Cells and stopping rules

- Ladder: p in {251, 509, 1021, 2039}; curves 0, 1, 2 (curve seed from base seed 20261121, p, curve index, trial; first trial with [pcc:unverified]
  N having a prime factor ell >= N/16 with ell^2 not dividing N); 3 DLP targets per curve (x random in [1, ell-1]). p = 4093 is attempted
  only if time allows and only IC-side (rho there is too costly: one add ~ 0.6 ms); it is not part of the 4-point fits.
- IC: for every arm, ladder of rungs 0..11 (RUNGS = 12; changed from 7 after a p = 251 smoke run showed alpha 0.5 and 2/3 ran out of rungs; no ladder data seen) stops at the first rung where all 3 targets are solved and verified; the ladder is run twice
  (A/A: second pass uses the continued RNG stream, i.e. a different F sample). Individual-log cap 60000 steps per record.
- Costs: one GAE = one combine (counted exactly). Native work = F_p multiplications (relation phase, FB sampling, support tests) plus
  mod-ell word operations (Wiedemann as in round 1 plus the filter's own counted operations from `jv_bitops`). Rho native = 1 per step + 1 per DP.
  Prices w = 0, 0.1, 1 GAE per native op, plus the calibrated w_cal = t(mulmod)/t(add) measured in the same binary (labelled).
  Per-target IC cost (primary) = ALL rungs' shared cost (relations, LA, adders, scale record) + the target's own cost; amortised over 3 targets shown separately.
- Rho: p = 251, 509, 1021: one run per target (9 per p); p = 2039: one run on target 0 per curve (3 runs; 28 min each). The rho reference at
  each p is the pooled mean of measured adds+setup; the model reference r * sqrt(ell) with r the pooled mean of steps/sqrt(ell) over all measured runs is shown beside it.
  DP bits = floor(log2(sqrt(ell)) - 4); cap 60 sqrt(ell) steps per run.
- Yield study: p in {251, 509, 1021, 2039}, 3 curves, phi in {0.3, 0.5, 0.7, 1.0}, F = D0 + random subset; at p <= 509 the round-1 scan runs on the same F as a cross-check.
- Fits: log-log OLS on the 4 ladder points (per-curve values, bootstrap over curves within each p, 2000 resamples, 95% percentile interval); exponent of
  the median-of-curves is also given. Fits are shown only when >= 4 points have a value (arms that fail at some p get no fit).
- Unit tests (must pass before measuring): root finder vs brute force; division by known roots; root-finder residual points = scan on p = 13..127 at three
  F densities (exact hit equality, anomalies = 0); ev1 coordinate order; LP-bearing relations true in Pic (p = 31); sampled FB points on the curve; round-1 tests unchanged.

## Declared limitations

- Split quadric only; one trigonal pencil pair; the factor-base size c0 and the rung multiplier were fixed from the heuristic above, not tuned on the ladder. [pcc:unverified]
- Decomposition cost is a scan over U (9 mulmods per point), not a polylog support extraction; its cost is part of the individual-log native count.
- Instance work (point counting, group order, curve enumeration to find D0 and fibre statistics) is uncharged for both arms.
- Dense kernel-dimension check only for <= 600 used columns; above that the verified final log is the check.
- Smoke runs at p = 31 and 61 during development before this plan was frozen are not used in any table.
