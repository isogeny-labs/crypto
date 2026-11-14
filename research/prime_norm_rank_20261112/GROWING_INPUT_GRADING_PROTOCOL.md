# Growing norm input grading: prospective proof contract

2026-11-14. Continue the public prime-field non-generic sub-exponential
expected bit-cost proof objective. The completed finite count is source
740732e95389e5185ed9a6547c18d8d889a6c580. This next obligation binds an
actual growing input family to weights; it does not assume a small closure
cutoff, successful support, supplied normal form, or efficient finder.

Fix arity a>=2, residual support degree m=2a-1, and retain a-1 coefficients
A_j for j=1,...,a-1 and a-1 coefficients B_j for j=0,...,a-2. Over the
literal multivariate polynomial coefficient ring over a field K, define

    D(X)=X^a + sum_{j=1}^{a-1} A_j X^j,
    B(X)=sum_{j=0}^{a-2} B_j X^j,
    A(X)=D(X)+z*B(r)-D(r), f(X)=X^3+alpha*X+beta,
    N=A^2-f*B^2, U=N divMonic(X-r), R=P modMonic U.

Public alpha,beta,r,z and P's coefficients are scalars. The curve premise
z^2=f(r) binds U to exact norm division; the quotient and remainder must
be computed operations, not supplied witnesses. For a=3 this must bind to
the unchanged actual four-variable source with weights (4,2,3,1).

Assign w(A_j)=2(a-j), w(B_j)=2a-3-2j. Prove positivity and that this is
exactly the consecutive multiset 1,...,2a-2. Define a polynomial coefficient
budget by requiring weight(e)+2j<=d for every exponent e in coefficient j's
actual multivariate support. This avoids treating a zero coefficient as a
nonzero monomial of degree0. Target universal statements:

- actual N has coefficient budget4a;
- actual monic U has degree m and coefficient budget2m;
- monic division by such a U preserves a dividend's coefficient budget;
- if public P has degree<=B, every R coefficient satisfies the literal
  budget2B (hence its weighted variable degree<=2B).

All statements require exact multivariate support binding; agreement only
on finite-field points or a scaling identity alone is insufficient. A
proof using an explicit homogenizing polynomial must establish its faithful
connection to weighted monomials. Retain all remainder equations.
No numerical degree29 claim or original W-jet map follows from this contract.

Acceptance: compile without holes/new axioms, study gate and import-inclusive
fresh replay, exact declaration pins and unsigned PCC records, editable
source-linked report/diagram and inspected PDF. Preserve failed attempts
and old evidence. No solver, native fixture, paid/cloud/attack/F4/msolve,
Python cryptomath, vault, signature, ledger rewrite or gate bypass.

Sources checked(KB): unavailable; bounded primary fallback read pinned
Mathlib weighted homogeneous definitions/Finsupp weight/support multiplication
and actual Div.lean well-founded monic division recursion, and existing
NormMembershipElimination/NormPoleTransport/NormSplitBaseDescent. Prior
WEIGHTED_LEADING_PROTOCOL records the genericity and weight hypotheses of
the weighted-GB literature. No claim that this new obligation is already
proved or novel. Conductor executable absent. Scope new proof/control/report
paths under the current study, its manifest, journal and append-only claims.

A completed input budget does not imply that full iterated closure remains
within it. The next discriminating question is an actual constructive
closure/normal-form bound for this growing family; a small zero set or
low displayed input degree cannot substitute for it [pcc:unverified].
