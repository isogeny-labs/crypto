# Nonlinear coefficient-domain constructor

For a caller-certified odd prime r, put h=(r-1)/2 and M=X^r-X.
The algebra A=F_r[X]/(M) is a ring of polynomial functions, not an extension
field: M splits into r distinct linear factors. Every function has a unique
representative of degree below r. Euler's criterion represents chi(X+a)
by (X+a)^h, including the zero at -a.

For every subset S of {0,...,q-1} of size at most three, construct the
coefficient column of P_S=product_{a in S}(X+a)^h mod M. The empty subset
is 1. Solve C c=e_1 in coefficient coordinates. Exact recomposition to X
modulo M certifies sum_S c_S product chi(X+a)=X at every field input.
The constructor uses no field-evaluation table, old packet coefficients,
scalar labels, source calls, or elliptic target. It receives r and q only,
plus explicit maximum row/column limits. Primality is a caller obligation;
pivot inverse checks are not primality certificates.

## Equivalence and scope

Let V be evaluation at all r field elements on the degree-below-r monomial
basis. V is an invertible Vandermonde matrix. The old value-domain feature
matrix is V C, and its target vector is V e_1. Thus the two representations
have identical solution sets, rank and consistency. The old value-domain
constructor also used public field coordinates, not secret scalar labels.
Changing basis does not reduce the number of unknowns or equations by itself.
This component establishes a native coefficient-domain reference and exact
saved-packet certificate, not a compressed solve or new elimination method.

## Implementation and retained costs

lib.rs builds shifted powers by repeated linear multiplication, then uses
ordinary quadratic polynomial products with X^k -> X^(k-r+1) for r<=k<=2r-2.
The constant term is never folded into degree r-1; this matters at X=0.
It constructs all square-free features up to degree three in deterministic
order, runs exact forward elimination and back substitution with free
coefficients zero, and recomposes every polynomial coefficient before return.
Inconsistent systems return OutsideSpan with the observed rank; there is no
implicit repair, row omission, field search or fallback packet.

There are m=1+q+binom(q,2)+binom(q,3) columns. This dense reference uses
O(m r^2+q r^2) polynomial-build work and O(r^2 m) elimination work, plus
O(r m) field slots. Its source-bound operation counter records only inner
elimination updates; it excludes polynomial construction, pivots, inversions,
back substitution, certificates, allocation, and source evaluation.

The r479/q16 solve has 697 columns, observed rank479, 334342 augmented
matrix entries, 478 nonzero output terms and 56196286 elimination updates.
The matrix payload is 2674736 bytes, excluding row/container overhead.
A separate original-column payload of 2670904 bytes is also retained during
the solve, plus shifted powers and scratch. These are derived payload sizes,
not RSS. The generated packet still requests 16 source characters and has
one more nonzero term than the preserved 477-term packet. No timing or
whole-recovery improvement is asserted.

## Exact checks and independent artifact replay

A bounded optimized overflow-checked native build/run passes all 47 tests,
none filtered. The r479/q15 negative control returns OutsideSpan with observed
rank477. The q16 construction succeeds, recomposes exactly to X, checks all
479 inputs using a separate quadratic-residue enumeration, and passes eight
saved public-point source integrations through the centered decoder. It does
not replace the production recovery API or its independent group verifier.
Small controls cover a consistent F7 cubic case, insufficient span, domain
and allocation limits, and monomial quotient products at every input for
r in {3,7,11,31}. The imported source/decoder controls remain intact.

check.rs imports no producer, solver, source or decoder. It reads the saved
program.tsv, validates canonical coefficients and distinct sorted labels,
expands each ordinary polynomial by repeated linear factors without quotient
reduction, then folds the complete sum once by e -> 1+(e-1) mod (r-1) for
positive e. Its exact coefficient vector is X. It separately verifies all
479 inputs using a quadratic-residue bitmap. A saved packet with its first
coefficient changed is rejected at the polynomial-identity assertion.
The deliberate failure log is preserved as a negative control.

Eighteen producer source/data hashes verify before and after build. Three
checker/input hashes bind the independent replay. Four producer compiler
warnings remain. Native commands, timeouts and binary hashes are retained in
receipts. No cloud job or timing panel ran; the original 4563-case cohort is
unchanged. Observed ranks are exact backend outputs, not an independent
rank-certificate replay. The independent checker certifies the target identity.

## Next structural step and literature

NEXT_RECURRENCE.md derives a short power-coefficient recurrence with a
Frobenius anchor at its characteristic-r singularity. That proposed builder
has not been implemented or measured in this component. It can be checked
against this frozen dense backend without changing the elimination problem.

Reviewed the existing consistent-repair and coefficient-Schur derivations.
Standard quotient-ring and linear-system semantics were checked in the
SageMath documentation (read 2026-11-05):
https://doc.sagemath.org/html/en/reference/polynomial_rings/sage/rings/polynomial/polynomial_quotient_ring.html
https://doc.sagemath.org/html/en/tutorial/tour_linalg.html
No novelty claim is made for quotient-ring arithmetic or Gaussian elimination.
