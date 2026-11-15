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

## Limits

One prime, n <= 16, sympy timings on a shared host, the group has no large
prime-order subgroup, few relations exist at this size. Reproduce:
`python3 -I alg.py; python3 -I gb.py` (sympy 1.14).
