# The original quadratic source quotient

2026-10-28 UTC. Native polynomial certificates and written geometric
interpretation are distinguished below. Fixed inputs: the unchanged Round22
incidence circuit and literal generic nu equation. Let A=1+c,
Delta=b^3*(b+c*A^3), and L be the retained degree16 source field over
F2(E), E:v^2+v=u^3+u. This is the incidence model; no independent X1(77)
identification or desired modular-factor map is inferred.

## Actual original-coordinate action

Deck25-positive-04 checks the F2 polynomial action
sigma(nu,lambda,b,c)=(nu+b,lambda+A,b,c). Its square is identity. Each of
the five original residuals nn49,nn53,nn239,nn241,nn243 and each of the eight
actual registered entries nn1,nn12,nn2,nn2,nn20,nn37,nn65,nn75 is exactly
preserved, before reduction by an ideal. The literal generic nu equation
and its coefficient D=A^2*Delta are preserved too. The three invariants are

    Z=b+lambda^2+A*lambda,
    U=A*nu+b*lambda,
    V=nu^2+b*nu.

The accepted native reductions give U=N2/(A*Delta), V=N3/(A^2*Delta),
with both numerators of degree7 in Z. Their full retained expressions are
in quotient-coordinates.sing, copied byte-for-byte from the accepted dump.
Cleared source identities and the mixed norm relation reduce to zero modulo
lambda^2+A*lambda+Z+b and the retained monic H(Z). The displayed denominators
expand to A*Delta and A^2*Delta; these are generic formulas, not local
regularity claims at A=0 or Delta=0.

Round22 already proves [L:F2(E)(Z)]=2. The nontrivial sigma fixes Z and the
base; A is a nonzero generic function. Thus the fixed field is exactly
K=F2(E)(Z). This uses the actual source field, not equality of L-polynomials.
The extension and action are defined over F2. The field-theoretic bridge is
written; the fourteen Lean theorems prove the universal algebra listed in
PROOF_MAP.md, rather than formalizing this identification of function fields.

## Complete quotient presentation on the registered b-unit open

Q25-registered-05 retains thirteen explicit polynomials Q1,...,Q13 in
F2[U,V,b,c]. For a source polynomial P with lambda-degree d it certifies

    Q(A*nu+b*lambda,nu^2+b*nu,b,c)=b^d*P(nu,lambda,b,c)

as an exact polynomial identity, with no reduction in the original ring.
The source indexes are [49,53,239,241,243,1,12,2,2,20,37,65,75], and the
b-powers are [0,0,6,7,8,0,0,0,0,0,0,3,2]. No residual nu coefficient remains
when reducing by the monic relation nu^2+b*nu+V. The full thirteen polynomials
are in registered-quotient.sing, copied unchanged from the accepted dump.

Define B to be F2[U,V,b,c]/(Q1,...,Q5), localized at Q6*...*Q13. In
particular b=Q6 is a unit. There is an explicit isomorphism between the
original registered source algebra and

    T=B[nu]/(nu^2+b*nu+V),       lambda=(U+A*nu)/b.

Indeed the forward coordinates U,V and the inverse lambda obey their
identities in either presentation. The thirteen exact pullbacks identify
both ideals and localizations because b is a unit. Hence this statement
keeps the entire registered affine scheme, including any components or
nilpotents it originally has; it does not assume an unproved saturation.
The monic presentation makes T free of rank2 over B. Its derivative b is a
unit, so it is finite etale. The invariants are exactly B: every element
of T has a unique form p+q*nu, and sigma sends it to p+q*nu+q*b; invariance
forces q=0. This also proves injectivity of B into T.

This construction never divides by A. It therefore remains a valid
registered-open chart at A=0, including the Round23 SS5 neighborhood. It
does not give equations for the entire proper boundary or resolve the
separate coupled Kummer quotient and proper polarized embedding problem.
The source-field involution extends to its proper normalized curve by the
function-field/normal-curve correspondence, but its boundary ramification
and a projective equation model have not been calculated here.

Two last registered entries have particularly short quotient expressions:

    b^3*S = b*U^3+b^2*A*U^2+b*A^2*U*V+b^4*U+b^3*V,
    b^2*N = b^2*c*A*U^2+b^5*c.

These are the actual nn65 and nn75 entries, not surrogate denominators.

## Direct norm formulas and a discriminating next check

For the proper degree2 map pi:C->H induced by K subset L, the actual
quadratic trace/norm formulas are

    Tr(lambda)=A,     Norm(lambda)=Z+b,
    Tr(nu)=b,         Norm(nu)=V,
    A^2*V=U^2+A*b*U+b^2*(Z+b).

The Lean identities and native cleared checks support these formulas. A
written extension for a general element p+q*nu with p,q in K is
Norm(p+q*nu)=p^2+b*p*q+V*q^2. This gives a concrete function-norm route once
divisor arithmetic on H is available; it is not an arbitrary-divisor class
reverse evaluator. On Picard groups pi^* pi_*=1+sigma, so the first required
test is sigma's action on the desired order11 factor. If it acts as -1,
this quotient norm kills that factor. If the plus part is nonzero, the
projector 6*(1+sigma) modulo11 isolates it, but an actual factor map and
nonzero subgroup certificate are still required. No such action is measured
or inferred here, and no relation-rank or IC cost change is claimed.

## Sources and scope

Before the method was frozen: Round22 CM/tower/generic certificates,
Round23 CM and Round24 CM/NEXT/results, and existing R21/R22 proof maps.
Primary sources read: [Stacks09I7](https://stacks.math.columbia.edu/tag/09I7)
for Artin-Schreier extensions; [Stacks03PA](https://stacks.math.columbia.edu/tag/03PA)
for etale presentations; [Stacks0BXX](https://stacks.math.columbia.edu/tag/0BXX)
for normal curves and function-field maps. Their framework informs the
written interpretation; native pullbacks are specific to our unchanged
source. No novelty, independent scientific-review or full Lean geometry
claim is made. R22 recovery d=1 and all prior controls remain frozen.
