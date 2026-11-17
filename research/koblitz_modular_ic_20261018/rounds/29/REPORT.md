# R29 observations: the double image and a full-level cubic bridge

These are exact native computations, kernel-checked algebra and labelled
written geometric deductions. No IC efficiency or independent relation-rank
assessment is made.

The degree-12 support has multiplicity two and both first adjugates vanish.
R29 computes the epsilon² coefficient of Norm(F+epsilon H), epsilon³=0,
for H=1, x_infinity and 1+x_infinity. Native truncated-series inverses,
multiplicativity and two planted norm controls pass. Every zeroth coefficient
matches M15; every first coefficient matches R28. All 1,024 coefficients of
all three second-order functions are recovered in the same base RR space.

If the unweighted second coefficient is C, the weighted ratios give product
p=C_x/C and sum s=C_(1+x)/C+1+p in characteristic two. At the actual support
C is nonzero. The resulting image polynomial is

    X² + beta^11 + beta^8 + beta^5 + beta^3 + beta.

It is the square of X+beta^11+beta^8+beta^5+beta^3+beta^2+1 in the registered
F2^12 field. Exactly one of the three actual cubic roots is selected, with
multiplicity two and closed degree 12. Thus the ordinary paired-image data
is resolved for all four first supports contributing degrees 460 and 367.
This identifies the image multiset; it does not distinguish every original
source branch or construct its colored C5 lift.

The completed dependency calculation gives the following exact offsets,
where -1 means no match anywhere in the complete Frobenius orbit:

| First degree | Multiplicity | Frobenius | Negation | Translation by T | Negation + T |
|---|---:|---:|---:|---:|---:|
| 436 | 1 | 218 | -1 | -1 | -1 |
| 12 | 2 | 6 | -1 | -1 | -1 |
| 284 | 1 | -1 | -1 | -1 | -1 |
| 83 | 1 | -1 | -1 | -1 | -1 |

Both translations and their involution identities are checked on the actual
curve, along with ordinary cusp/SS avoidance, a corrupted-ordinate detector,
and all five coefficient-descent curve/product conjugates. The ordinary
460 contribution cancels as a BASE E49 closed divisor. The asymmetric 367
supports survive these particular base-image coincidence tests. The colored
C5 deck offsets and modular-factor columns are still required; neither a
zero nor a positive IC rank follows from this table.

A separate native check pursues the proper level-77 model relationship.
The actual degree-14 lambda-zero point gives an irreducible, separable
QR-intersection cubic. Exact inverse/lift certificates and Tate group laws
produce a point of exact order 77 over F2^42, with CRT projectors 56 and 22
and diamond action 23 of order three. The monic/DVR specialization argument
then proves that adjoining a QR generator is generically degree three.
MODULI_BRIDGE.md states the written identification with X1(77) and its
proper diamond quotient; it is not replaced by finite-point or genus matching.
The independent [Sutherland/van Hoeij model](https://math.mit.edu/~drew/X1/X1opt77new.txt)
is archived with its hash. Its global rational-function match is not checked.

All four native runs and the bounded compiler succeed. Lean compiles and
audits the two universal pair-sum/polynomial identities, zero failures and
zero trusted-native. Across R27–R29 there are 40 named theorems and 41 current
audited declarations, counting one generated congruence theorem. The final
retained-record checker verifies 21 accepted native runs among 32 attempts
and all current proof audits. This is source/record integrity verification,
not an independent mathematical replay.
