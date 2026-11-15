# Symmetrized m = 4 decomposition: multiplicative-subgroup factor base

Question: does a factor base equal to the n-th roots of unity mu_n (binomial
X^n = 1, n | p-1) give Groebner decomposition an advantage over a dense
small-x factor base, once the summation polynomial is symmetrized?
Exploratory msolve 0.6.5 runs, not bench.record (no S3 credentials in the
session); all numbers [pcc:unverified].

Setup (`m4_sym.py p n [--ds]`): E: y^2 = x^3 + 5x + 7, target R decomposed as
P1 + P2 + P3 via S4(X1,X2,X3,xR) = 0 (deg 4 per variable, 439 terms);
p ~ n^3/3 with p = 1 mod n; 8 targets, arm order shuffled per target; time =
min of 3 msolve wall times minus a trivial-system baseline. Checks asserted
every target: symmetrization exact, Newton sums checked numerically, each
basis is [1] iff brute force finds no solution.

Arms: D dense unsymmetrized; Bn binomial unsymmetrized; Bs binomial
symmetrized (e3^n = 1, p_n = 3, e2(X^n) = 3); Ds dense symmetrized
(sigma_k of F(X_i), n <= 16 only; its sympy build time is excluded).

| p | n | D | Bn | Bs | Ds | distinct relations dense / binomial |
|---|---|---|---|---|---|---|
| 193 | 8 | 15 ms | 15 ms | 6.2 ms | 7.2 ms | 31 / 9 |
| 577 | 12 | 74 ms | 82 ms | 28 ms | 32 ms | 35 / 3 |
| 1361 | 16 | 264 ms | 244 ms | 102 ms | 107 ms | 17 / 2 |
| 2741 | 20 | 721 ms | 649 ms | 264 ms | - | 28 / 7 |
| 4657 | 24 | 1720 ms | 1598 ms | 626 ms | - | 31 / 10 |
| 9001 | 30 | 4564 ms | 4474 ms | 2012 ms | - | 26 / 5 |

Findings [pcc:unverified]:

- Symmetrization cuts solve time about 2.3-2.8x for either factor base.
  Where both symmetrized arms exist (n <= 16), Bs and Ds are within 5-15%.
  The gain is from symmetrization, not from the subgroup structure.
- The binomial base yields 3-12x fewer distinct relations: only about half
  of mu_n are x-coordinates of F_p points, so about 1/8 of triples are valid.
- Time per relation (median x 8 targets / relations): Bs is 2-10x worse than
  Ds at n <= 16, and worse than even unsymmetrized D at every n.
- Verdict: no advantage for a multiplicative-subgroup factor base at m = 4.
  The idea is not supported at these sizes.

Errors fixed while building: (p_n^2 - p_2n)/2 coerced to a rational (now uses
2^-1 mod p); sympy `symmetrize(formal=True)` returns three values; relation
counts initially mixed ordered triples and multisets.

Prior art: see `PRIOR_ART.md` (the subgroup factor base is PKM 2016 section 3.3).

Limits: one curve, n <= 30, single-threaded msolve on a shared host; Ds only
to n = 16.
