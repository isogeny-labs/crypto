# Algebraic index calculus with a linear two-summand decomposition: ladder protocol

Frozen 2026-10-24 before any ladder timing. Development runs (n = 17, 19, 23
on a = 1, seed 7, and the n = 29 stall that exposed the cofactor issue) are
excluded; ladder seeds below are fresh.

## Question

Owner criterion (2026-10-24): an index-calculus pipeline's relations must
come from solving summation-polynomial systems over an algebraically defined
factor base, without table collisions. The repository's algebraic arm
(ecbench `pdp3-koblitz`, inherited F4 on the Weil descent of S4 over
`span{1..z^{d−1}}`, m = 3) completed n = 13, 19, 23 on Koblitz a = 0, with
counted solver word XORs of 6.0e6, 5.4e8 and 1.06e10 per solve
(`research/f6_ic_ecbench_ladder_20261005/ANALYSIS.json`). Does the linear
two-summand decomposition (`examples/ic_m2_linear.rs`) lower the cost
exponent, and how far is it from same-target strong rho on each rung?

## Method under test

Factor base `{P : x(P) ∈ V \ {0}}`, `V = span{1, z, …, z^{d−1}}` in the
polynomial basis, `d = ⌊(n + 1)/3⌋`. A point R with known logarithm is
decomposed as `ε1 P1 + ε2 P2` by solving the Weil descent of
`S3(x1, x2, xR) = p² + xR p + xR² s² + b` (s = x1 + x2, p = x1 x2), which is
F2-linear in (s, p) ∈ V × span{1..z^{2d−2}}; each solution is split by the
linear system `x1² + s x1 = p` over F2[z]. Relations are verified in the
group. Linear algebra mod r is union-find over the relation graph;
balanced components are evaluated by cancellation. Model: about `2^{n−d+1}`
trials of polynomial cost, about `2^{2n/3}`.

## Rungs and runs

Koblitz curves from `KoblitzCurve::new(a, n)` with cofactor at most 4:
a = 0 at n = 13, 19, 23, 41; a = 1 at n = 17, 19, 23. Check rung with a
moderate cofactor: a = 0, n = 37 (h = 596). Targets: 8 per rung (4 at
n = 41 and 37), seed 20261024, one thread, each target a cold IC process
stage (factor base, relations, linear algebra, target) followed by the
strong-rho reference (`StrongRho`, signed Frobenius, 32 lanes, dp 4) on the
same point in the same process and the same `Gf2` field type.

## Measures and decision rule

- Per target: IC trials, relations, group additions, F2 elimination XORs,
  wall seconds; rho walk steps, group additions, wall seconds; both scalars
  checked against the secret.
- Exponent: least-squares slope of log2(mean IC trials) against n over the
  small-cofactor rungs, with a bootstrap over targets within each rung; the
  same for IC wall time and for rho group additions.
- **Improvement** over the F4 algebraic arm if the IC trials slope is below
  the F4 arm's log2 solver-work slope over the same n range and the IC
  solves n = 41, which F4 could not (n = 31 timed out at 3600 s).
- The distance to rho is reported per rung as IC wall / rho wall and IC
  counted operations / rho group additions, with §5a pricing of the
  elimination XORs at 0, 0.1 and 1 GAE per word XOR.

## Limits stated in advance

Wall times are from one shared cloud container (L0). The counted IC unit (a
trial is about 3d field operations and two small eliminations) differs from
rho's (a group addition), so ratios in counts carry the §5a band. Large
cofactors fragment the relation graph (relations join only points whose
cofactor components cancel), so the method is not applicable as built to
the a = 1 N83 curve (h ≈ 1.1e9).
