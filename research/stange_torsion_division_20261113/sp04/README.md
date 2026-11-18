# STANGE-SP-04 toy check: summation polynomials over 2-isogenous curves

Tests record STANGE-SP-04 in `../PROPOSAL.md`. Scratch-scale exact algebra plus
sympy Groebner timings at one toy prime. Not a bench.record/v1 benchmark and not
an ecbench measurement; all statements below are [pcc:unverified].

Setup: p = 1009, E1: y^2 = x^3 + 5x + 967 with 2-torsion point x0 = 3, E its
Velu codomain y^2 = x^3 + 854x + 295 (#E1 = #E = 1036, checked by point count).

## Algebra (`alg.py`, output `alg.out`; exact over F_p, asserted)

- Norm over y-signs of the (1,1,1) net numerator on E = Vandermonde^2 * S3, quotient exactly 1 [pcc:unverified].
- Pulled back through phi: norm = prod (x_i - x0)^2 * (pulled Vandermonde)^2 * (pulled S3), exact [pcc:unverified].
- Degree per variable: S3 = 2, pulled S3 = 4 (13 vs 112 terms), pulled net norm = 14 [pcc:unverified].

The net polynomial adds no zeros beyond the summation polynomial; the cofactor
is the kernel factor and Vandermonde terms.

## Groebner (`gb.py`, output `gb.out`)

Median seconds over 6 targets, randomized arm order per target, sympy grevlex.

| n | plain small-x on E | plain x(phi(F1)) on E | pulled back to E1 |
|---|---|---|---|
| 4 | 0.004 | 0.004 | 0.009 |
| 8 | 0.015 | 0.014 | 0.107 |
| 12 | 0.036 | 0.037 | 0.320 |
| 16 | 0.059 | 0.060 | 0.654 |

The pulled-back arm is 2-11x slower at matched size, and its relations are
exactly the image arm's relations weighted by phi's 2:1 x-multiplicity
(asserted per target) [pcc:unverified]. Each arm's basis is [1] iff it has no
solution (asserted). Prediction of SP-04 holds at this scale; the record is not
falsified [pcc:unverified].

## Larger primes (2026-11-15)

`gb.py p 5 3` (curve family a = 5, x0 = 3; outputs `gb_p<p>.out`) and
`alg.py 1000003` (`alg_p1000003.out`; all identities hold exactly) [pcc:unverified].
Median seconds at n = 16 (6 targets):

| p | plain small-x on E | plain x(phi(F1)) | pulled back | ratio pulled/plain |
|---|---|---|---|---|
| 1009 | 0.059 | 0.060 | 0.654 | 11x |
| 10007 | 0.076 | 0.081 | 0.683 | 9x |
| 100003 | 0.075 | 0.080 | 0.704 | 9x |
| 1000003 | 0.075 | 0.077 | 0.663 | 9x |

The ratio is flat in p and grows with n (about 2x at n = 4) [pcc:unverified]. Caveat: at fixed
n the expected number of relations falls like n^2/p, so for p >= 10^5 no target
decomposed (0 solutions in every arm), and these cells time a proof of
infeasibility rather than a solve. A fair large-p test needs n near sqrt(p),
far beyond sympy; that would be a msolve/Magma run under bench.record.

## msolve at n ~ sqrt(p) (2026-11-15)

`msolve_sp04.py p 11` (msolve 0.6.5, one thread; outputs `ms_p<p>.json`). Primes
chosen with p = 1 mod floor(sqrt p) so arm B gets m = n. 8 targets, arm order
shuffled per target; time = min of 3 msolve wall times minus a trivial-system
baseline (~2 ms). Exploratory, not a bench.record run (no S3 credentials here).
All numbers [pcc:unverified].

| p | n | E plain small-x | I plain x(phi F1) | P pulled back | B binomial mu_n | relations E/I/P/B |
|---|---|---|---|---|---|---|
| 1123 | 33 | ~0.1 ms | ~0.1 ms | 1.1 ms | noise | 42/38/38/10 |
| 10303 | 101 | ~0.2 ms | ~0.3 ms | 5.9 ms | noise | 38/38/40/8 |
| 106277 | 326 | - | - | msolve segfault | - | - |
| 1003003 | 1001 | 1.8 ms | 1.6 ms | 1148 ms | 0.45 ms | 38/48/48/8 |

- The pulled-back arm's penalty grows with n: about 10x at n = 33, 20-30x at n = 101, 600x at n = 1001 [pcc:unverified]. SP-04 not falsified.
- Arm B (multiplicative-subgroup factor base): about 4x less solve time than E at n = 1001 but about 5x fewer relations (only about half of mu_n are x-coordinates), so time per relation is about equal (E ~0.39 ms, B ~0.45 ms) [pcc:unverified]. No gain at m = 3; the symmetrized m >= 4 version is untested.
- E, I and B times at p <= 10303 sit at the timing noise floor (some baseline-subtracted minima are negative); only the gap to P is meaningful there.
- p = 106277: msolve 0.6.5 segfaults on the pulled-back system (also with the input pre-expanded); input saved as `fail_p106277.ms`. Cause not confirmed.

## Errors found while checking

- An ad hoc point-count snippet (not in these scripts) reported #E1 != #E; it
  counted non-residues as one point. Corrected count: 1036 for both.
- The first `alg.py` draft cleared the 2B term of the pulled S3 with D1*D2
  instead of D1^2*D2^2; divisibility failed until fixed. Now cross-checked at
  random points against direct composition.
- An eyeballed cofactor evaluation at x2=7, x3=11 was zero only because
  phi_x(7) = phi_x(11) = 15; replaced by an exact identity assertion.
- Solution counts for image vs pulled arms differ (4 vs 6 at n=16) because
  phi is 2:1 on x; explained and asserted, not an error.

- First msolve attempt: p = 10007 gave arm B m = 2 (p-1 = 2*5003), invalid;
  the harness now refuses primes without a divisor near n. Timings below the
  process-start baseline were clamped to 0; now reported raw.

## Limits

One prime, n <= 16, sympy timings on a shared host, the group has no large
prime-order subgroup, few relations exist at this size. Reproduce:
`python3 -I alg.py; python3 -I gb.py` (sympy 1.14).
