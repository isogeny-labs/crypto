# Conic descent recovers the registered public logarithm

Two separately frozen Rust gates verify conic residual descent on the same
closed degree-three divisors produced by the algebraic inverse. All seven
accepted anchor pairs recover k=183049; each answer passes scalar multiplication
on the original genus3/F127 source curve. No expected scalar is supplied to
the logarithm gate. Classification: engineering implementation of classical
plane-curve descent plus a derived centering simplification, not a novelty,
complexity or timing claim.

## Exact workload and evidence

Source: y²=f(x), f ascending [31,89,73,48,91,41,123,1], p=127, ell=1028893;
D1=2*(0,44), B=2*(3,21). The target quartic has descending homogeneous
coefficients [1,58,103,47,66,88,51,80,84,123,34,83,109,1,111].
Closed graph inputs, fixed from the prior recovery:

- E0: u=[9,31,82,1], v=[50,30,116], public orientation positive.
- E1: u=[80,89,51,1], v=[97,30,21], public orientation negative.

The five public anchors are (18,52),(28,86),(43,125),(101,43),(34,113).
All ten unordered anchor pairs are examined for each E in fixed order. The
candidate solves a 5-by-6 conic kernel, evaluates nine 6-by-6 Sylvester
determinants, interpolates the degree-at-most-eight resultant and divides
exactly by the known degree-five intersection. Frobenius and bounded
factorization find the rational residual support without a field-point scan.
No Jacobian addition occurs in the conic/logarithmic descent candidate.
Setup and independent verifiers do use previously implemented group arithmetic.

| Outcome | E0 | E1 | Total |
|---|---:|---:|---:|
| Requested pairs | 10 | 10 | 20 |
| Accepted split residuals | 3 | 4 | 7 |
| Excluded nonsplit residuals | 7 | 6 | 13 |
| Distinct accepted conics | 1 | 4 | 5 |

The three E0 acceptances reuse one conic with different anchor pairs. These
are not independent trials, and 7/20 is not an estimated splitting probability.
All pairs have rank five and pass exact known-factor division. For each conic,
127 direct Sylvester evaluations check the interpolated resultant. All seven
accepted divisors satisfy the independent Jacobian group identity. Both
corrupted graph controls fail the curve condition. No scientific rerun.

Gate1 source333061c10519/checkpoint20261115T222341Z: one execution,
exit0/empty stderr, [RUN.txt](RUN.txt). Gate2 source442d2bb8fc0e/checkpoint
20261115T222548Z: one execution, exit0/empty stderr, [LOGS.txt](LOGS.txt).
The first log-gate compile used the wrong Poly constructor; its diagnostic
is retained and corrected before freeze. Native binary fingerprints, protocols
and compiler outputs are adjacent. Local macOS, no isolation or timing study.

## Why centering constants disappear from the descent

Let O=(18,52) and L(P)=log([P-O]) in the odd prime-order quotient, in the
scale of the existing point-log kernel. Put K0=log([K-4O]) and
D0=log([Dinf-3O]). The prior centering identity is
2 beta=2Dinf-2O-K, hence log(beta)=D0-K0/2.
For the lifted class X=[E-Dinf]+beta this gives

    log X = log([E-3O]) - K0/2.

A conic section has divisor E+A+B+R linearly equivalent to 2K. Therefore

    log X = 3*K0/2 - L(A) - L(B) - sum(P in R) L(P).

Division by two means inverse of two modulo ell, not a choice in the full
Jacobian's even torsion. The identities hold after projection to the checked
odd subgroup. They eliminate beta and Dinf group computations from logarithmic
descent; they do not eliminate the calibration/sign costs of the inverse.
The tangent line has K=2O+(101,43)+(34,113), so K0 is two point-log lookups.

The reused public generator calibration gives log G=47376. E0 gives
log X=666596; E1 gives log X=362297, its negative modulo ell. After the
already verified public sign and subtraction of G, all seven yield
log B=619220. Their quotient gives k=183049, verified by [k]D1=B on the
source. [POINT-LOG-RECORD.txt](POINT-LOG-RECORD.txt) contains only the earlier
point-log kernel, not the target scalar or target class logarithm.

## Scope and next technical step

This joins the actual inverse output to point-log descent without translation
search. Point-base construction/log solving and the public inverse sign are
reused; this is not an independent inverse replay or complete cold experiment.
Exceptional projection, repeated support and missing residual point logs must
stop or use a separately supported chart/descent. The present tests cover two
fixed closed divisors and five distinct accepted conics.

Existing performance graphs remain unchanged: there are no new comparable
wall-time measurements. [REPORT.pdf](REPORT.pdf) and [flow.svg](flow.svg)
explain the exact arithmetic chain. The older measured cold-cost failure
remains valid; full10x/genus2+ is open. Next integrate cold quotient construction,
source-coordinate evaluation, inverse/sign, this conic descent and sparse
point-log solving in one admitted pipeline. A larger-field cost case is needed
because the registered F127 baseline is already below a millisecond.

Primary comparators: [Diem2012](https://jtnb.centre-mersenne.org/articles/10.5802/jtnb.815/)
and [Tian section4.2.3](https://arxiv.org/html/2007.03172). The conic divisor
identity is standard geometry; no literature absence or mathematical novelty
claim follows from the corpus search.

## Formalization boundary

[Centering.lean](Centering.lean) states the scalar identity over any field with
2 nonzero, conditional on the centering and conic log equations. Its exact
hypotheses, toolchain and mathlib pin are in [PROOF-MAP.md](PROOF-MAP.md).
The final build and axiom output are in LEAN.txt/EXIT-LEAN.txt. This check does
not formalize the geometric hypotheses or the complete Rust implementation.
