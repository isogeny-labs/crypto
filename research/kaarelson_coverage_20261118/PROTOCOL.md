# Protocol — Kaarelson v2 Theorem 3 coverage reproduction (frozen before the run)

2026-11-18 UTC. Follows the merged `fast-ecdlp-v2-assessment` (PR #4672).
Scope: an exact, engine-free reproduction of the coverage quantities the paper
reports for m = 2 at three small primes. Not a solver, not a timing comparison,
not an exponent or novelty claim. `[pcc:unverified]`

## What is measured

Fix m = 2, so X = floor(p^(1/2)). For a nonsingular curve E : y^2 = x^3 + a x + b
over F_p and a dilation u in F_p*, the paper's isomorphic model is
E_u : y^2 = x^3 + u^4 a x + u^6 b with phi_u(x,y) = (u^2 x, u^3 y), and the factor
base is F^(u) = { P in E_u(F_p) \ {O} : x(P) in [1, X] }. Because phi_u is an
F_p-isomorphism, pulling back to E gives, with s = u^2,

    B_u = { Q in E(F_p) \ {O} : (s * x(Q)) mod p in [1, X] },   f^(u) = |B_u|.

A decomposition R = P_1 + P_2 is **generic** (paper Definition 3.1, m = 2) iff
P_1, P_2 are affine with distinct x-coordinates. Measured quantities, per dilation:

- **coverage(u)** = |{ R in E(F_p) : R = P_1 + P_2, P_1,P_2 in B_u, x(P_1) != x(P_2) }| / n,
  where n = |E(F_p)|. (O cannot be a generic 2-sum, so it is never covered.)
- **additive energy** E_2(F^(u)) = #{ (P_1,P_2,Q_1,Q_2) in (B_u)^4 : P_1+P_2 = Q_1+Q_2 }
  = sum over group elements t of r(t)^2, r(t) = #{ (P_1,P_2) in (B_u)^2 : P_1+P_2 = t }
  (all ordered pairs, including doublings and inverse pairs). Reported as E_2 / X^2.
- f^(u) / X.

Averages are the **mean over all dilations u in F_p***. coverage, E_2 and f^(u)
depend on u only through s = u^2, and u, p-u share the same s, so the mean over all
u equals the mean over u = 1 .. (p-1)/2 (each quadratic residue s once). The value
at **u = 1** (s = 1), the paper's plain-box open problem, is recorded separately.

## Cohort and curve selection (deterministic)

- Primes: p in {1009, 4099, 10007}.
- **Generic arm (a != 0).** The first (a, b) in the order a = 1,2,... (outer),
  b = 1,2,... (inner) with a != 0, b != 0, nonsingular (4 a^3 + 27 b^2 != 0 mod p),
  and |E(F_p)| prime.
- **j = 0 arm (a = 0).** The first b = 1,2,... with b != 0 and |E(F_p)| prime, on
  y^2 = x^3 + b. This exists only for p = 1 mod 3; p = 10007 = 2 mod 3 gives
  supersingular order p+1 (composite), so the j = 0 arm is reported as unavailable
  at prime order for p = 10007 and is measured at p = 1009, 4099 only. The paper
  notes the a = 0 family gives the same numbers.

Prime order makes E(F_p) cyclic; points are represented by their discrete log to a
fixed generator, so a sum is an integer sum of exponents mod n and all counts are
exact integer histograms. |E(F_p)| is computed exactly as p + 1 + sum_x legendre(x^3+ax+b).

## Checks and observables

- Confirm the selected curve is nonsingular and |E| prime; confirm the generator
  walk visits n distinct points and that x(g^e) = x(g^(n-e)).
- Per arm: X, n, f^(u)/X (mean), coverage mean, SD and min/max over dilations,
  coverage at u = 1, and E_2/X^2 mean. A sanity identity: for every dilation,
  sum_t r(t) = f^(u)^2 and E_2 >= f^(u)^2 (Cauchy-Schwarz lower bound).
- Compare the generic-arm mean coverage against the paper's 0.39-0.42 and the mean
  E_2/X^2 against the paper's ~4, and report the plain-box u=1 vs mean-over-u gap.

## Reproducibility

Single native C++20 program (`coverage_t3.cpp`), no Python in the measurement.
Deterministic: the whole computation is an exact integer count, so a rerun is
byte-identical. Build and run commands and the output SHA-256 are recorded in
RESULT.md. Reports, a graph and a PDF follow the research-visuals skill.
