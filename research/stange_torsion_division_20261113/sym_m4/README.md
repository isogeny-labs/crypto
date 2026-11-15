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

## APS split-variable S4 encoding (2026-11-15)

Amadori-Pintore-Sala (FFA 2018, section 3.4) replace S4 = 0 with
S3(X1, X2, Y) = 0 and S3(X3, xR, Y) = 0 using an auxiliary variable Y. Arms Dy
(dense) and By (binomial) use that encoding with the same factor bases as D
and Bn; Res_Y of the two S3 is S4, so the "[1] iff no brute-force solution"
assertion still applies and passed on every target. Run:
`m4_sym.py p n [--ds] --arms=D,Dy,Ds,Bs,By`, outputs in `split/`. Targets differ
from the first grid (more arms shuffle the RNG differently). All numbers [pcc:unverified].

| p | n | D (S4) | Dy (split) | Dy/D | Ds | Bs | By (split) |
|---|---|---|---|---|---|---|---|
| 193 | 8 | 16 ms | 27 ms | 1.7 | 7.2 ms | 6.7 ms | 25 ms |
| 577 | 12 | 83 ms | 178 ms | 2.2 | 35 ms | 29 ms | 163 ms |
| 1361 | 16 | 267 ms | 573 ms | 2.1 | 103 ms | 100 ms | 530 ms |
| 2741 | 20 | 694 ms | 1784 ms | 2.6 | - | 269 ms | 1744 ms |
| 4657 | 24 | 1642 ms | 4175 ms | 2.5 | - | 632 ms | 3986 ms |
| 9001 | 30 | 4635 ms | 12700 ms | 2.7 | - | 1940 ms | 11495 ms |

- With msolve 0.6.5, the split encoding is 1.7-2.7x slower than S4 directly, and the gap grows with n [pcc:unverified]. It is 4-7x slower than the symmetrized arms.
- Not tested: APS also give each variable its own disjoint factor base f_i (smaller degree per variable). This harness gives every variable the same base. APS used Magma, not msolve.

## Speeding up the decomposition step (2026-11-15)

New arms (`m4_sym.py ... --arms=D,Ds,Dp,M` and `--arms=M,Mt`; outputs in `speed/`):

- Dp: APS-style disjoint factor bases. The n-element dense base is split round-robin into 3 parts, one per variable (degree about n/3 each). Only P_i in part i counts.
- M: hash join. Hash x(P1 +- P2) over pairs and match against x(R -+ P3); about n^2 point additions per target, in pure Python.
- Mt: M with the pair table built once per factor base and reused across targets; about 2n additions per target.

M and Mt return exactly the brute-force set of valid relation multisets (asserted every target). Dp keeps the "[1] iff no brute-force solution" assertion. Time per relation is median time x 8 targets / relations found. Numbers [pcc:unverified]; msolve arms are wall time minus baseline, and M/Mt are in-process Python.

| n | D | Ds | Dp | M | Mt | relations dense / Dp | table build |
|---|---|---|---|---|---|---|---|
| 8 | 5.6 ms | 2.5 ms | noise | 0.02 ms | <0.01 ms | 20 / 3 | 0.12 ms |
| 12 | 17.7 ms | 6.7 ms | 1.3 ms | 0.02 ms | <0.01 ms | 36 / 11 | 0.14 ms |
| 16 | 97 ms | 41 ms | 5.1 ms | 0.07 ms | 0.01 ms | 19 / 5 | 0.27 ms |
| 20 | 170 ms | - | 15 ms | 0.07 ms | 0.01 ms | 29 / 4 | 0.50 ms |
| 24 | 429 ms | - | 20 ms | 0.10 ms | 0.02 ms | 28 / 7 | 0.52 ms |
| 30 | 1194 ms | - | 59 ms | 0.16 ms | 0.01 ms | 28 / 7 | 1.50 ms |

(The table entries above are time per relation.)

- Partitioned bases (Dp) are the best Gröbner encoding tested: per relation about 5-8x cheaper than symmetrized (Ds) and about 13-20x cheaper than plain S4 [pcc:unverified]. They lose yield (about 1/6 as many relations), but per-target time drops more than that.
- Hash join beats every Gröbner encoding by 2-4 orders of magnitude at these sizes. The amortized table (Mt) cuts per-target cost a further 3-13x [pcc:unverified].
- This is not an index-calculus improvement against rho. With n^3 ~ p, Mt costs O(n) per target and decomposes with probability about n^3/p. Collecting about n relations then costs on the order of p/n, plus n^2 for the table, so about p^(2/3) group operations at best. Pollard rho needs about p^(1/2). The hash join only shows how slow the Gröbner encodings are in absolute terms at toy sizes. Gröbner-based decomposition only matters if its cost grows more slowly than n per target, and nothing here shows that.

Errors fixed while building: (p_n^2 - p_2n)/2 coerced to a rational (now uses
2^-1 mod p); sympy `symmetrize(formal=True)` returns three values; relation
counts initially mixed ordered triples and multisets.

Prior art: see `PRIOR_ART.md` (the subgroup factor base is PKM 2016 section 3.3).

Limits: one curve, n <= 30, single-threaded msolve on a shared host; Ds only
to n = 16.
