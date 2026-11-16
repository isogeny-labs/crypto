# CT22: direct elliptic norms for the actual chain evaluator

Prospective arithmetic engineering after CT21. No direct-norm adapter or its
tests have run. Preserve the existing norm implementation as the reference;
do not change the frozen chain proof, old snapshots, records or identity
proof-source pointers. New arithmetic and algebra checks use Rust/Lean.
No benchmark, solver or paid allocation without successful admission and
live budget/capacity. [pcc:unverified]

## Why this is the next task

CT21 now has actual Psi:T0→T1, with both full12317-class rational corpora,
global quotient/descent/polarization/noniso proof and all109 source controls.
The rational evaluator already moves division into elliptic coordinates.
Its N0/N1 still call the generic divisor-norm path. Derive direct base-field
formulas on balanced degree≤2 supports, preserving every exceptional case.
This proposes less arithmetic; it is not a measured improvement or an
asymptotic ECDLP claim. [pcc:unverified]

## Exact proposed domain

Start only with the actual monic even C0/C1 models overF101:

    C: y²=x⁶+d*x⁴+c*x²+b, b≠0,
    C0: (b,c,d)=(56,13,6), C1: (68,42,26).

Use existing Class/Balanced model/field/canonical validation. The first long
quotient is y²=U³+dU²+cU+b, with short shift s1=d/3 and point
(X,Y)=(x²+s1,y). The second is W²=V³+cV²+b*d*V+b², shift s2=c/3,
with (X,Y)=(b/x²+s2,b*y/x³). Preserve the actual C1 component swap/scales21
when returning matched109/113 coordinates. No unsupported general field,
nonmonic or catalog coverage is implied. [pcc:unverified]

## Proposed quadratic-support formulas, to be checked

Write monic u=x²+A*x+B and v=V0+V1*x. For a pair whose elliptic images
are (lambda*x_i²+s,V0+V1*x_i), a proposed symmetric addition formula is

    m=−V1/(lambda*A),
    Xsum=m²−lambda*(A²−2B)−2s,
    Ysum=m*(s−lambda*B−Xsum)−V0,

when A≠0. It should handle repeated support through the existing valid
Mumford jet relation; prove this instead of assuming the roots are distinct.
If A=0, validity on the even curve should force V0*V1=0: V1=0 gives
twice(X=−lambda*B+s,Y=V0), otherwise opposite points sum toO. Retain
the x=0 repeated case and zero ordinates. [pcc:unverified]

For the second quotient with B≠0, reciprocal roots t_i=1/x_i have
A'=A/B, B'=1/B. Reducing b*(V0*t³+V1*t²) modulo t²+A't+B' proposes

    V1'=b*(V0*(A'²−B')−V1*A'),
    V0'=b*(V0*A'*B'−V1*B').

Use the same symmetric formula with lambda=b, s=s2. Prove the polynomial
remainder identity and actual source/target curve equations in the stated
domain. No denominator may be inverted before its nonzero case is checked.
[pcc:unverified]

If B=0 and A≠0, one support root is0 and maps to the elliptic origin;
evaluate the other at x=−A. If both vanish, both second-quotient images
are the origin. For degree1 support, evaluate its actual point, treating
x=0 as origin on the second quotient. Degree0 has no finite contribution.
Use existing elliptic point/add/double routines; do not write another field,
elliptic or Cantor engine. [pcc:unverified]

## Balanced infinities are required

The first quotient sends both source infinities toO. The second sends
infinity± to ±Q, Q=(s2,b) on the short curve. For finite degree m and
plus-infinity count n, add (2n+m−2)Q to the finite second norm. This
includes degree0 plus counts0/1/2, degree1 and infinity differences. Compare
with the existing norm on the actual identity and every exceptional class;
do not silently restrict to generic degree2 supports. [pcc:unverified]

## Acceptance and continuation

1. Preserve a pre-implementation worklog and frozen input/source identity.
2. Prove feasible generic cleared support/slope/remainder/infinity lemmas
   faithfully in a NEW Lean file; record the missing Picard bridge explicitly.
3. Compare both direct norms against existing norms on ALL12317 C0 and
   ALL12317 C1 rational classes. Count every special case; include repeated
   support, zero coordinates, zero ordinates and all infinities.
4. Check point validity, direct sums, model rejection and actual matched
   quotient coordinates. Require all109 source controls and direct
   elliptic/reference endpoints.
5. Only then integrate the direct coordinates into the actual Phi/Psi
   evaluators. Keep old independent evaluation paths, full-domain dual7/
   bijection/addition checks and source recoveries. Required final checks
   must run on the changed source; do not use old passes as new-head CI.
6. Preserve failures and append new PCC records before citing results;
   publish source/report/diagram/inspected PDF and a ready stacked PR.
7. Then continue the next unblocked task: consider a further nondual odd hop
   only with genuine new target/kernel/global proof and direct elliptic
   controls. Avoid repeating the closed rational Richelot backtrack search.
   [pcc:unverified]

No held-out selection or speed protocol is frozen by this correctness plan.
If admission returns, preregister the actual matched timing protocol including
construction, transport, solve, recovery, verification and direct elliptic/
rho/BSGS/index-calculus controls. Root and consumer immutable-ledger forks,
archive-clock evidence and KB/vault integration remain unresolved separately.
[pcc:unverified]
