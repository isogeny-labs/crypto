# Explicit infinity charts and constant-field check

This is written geometry supported by native source identities. It is not a
formal Lean construction of a normalization, nor an evaluated modular factor
map. Keep the same incidence field L/F2(E) of degree16 as R22/R23, with
E: v^2+v=u^3+u and C its smooth proper model. No independent modular model
is substituted for this source field.

## A base chart at O in the actual Tate coordinates

Put t=u/v and q=1/(t^3*v). Clearing the cubic gives

    E0=q+1+(1+t)*t^3*q^2=0.

At O, t=0,q=1 and the q derivative is1. Thus t is a uniformizer on E.
The rational inverses are u=1/(t^2*q),v=1/(t^3*q). Define

    cp=1+t+t^4*q+t^6*q^2,  rp=1+t^2*q+t^5*q^2,
    c=t^2*q^2*cp,          b=t^3*q^3*rp*cp.

O24-charts-02 checks both original genus-one b,c substitutions modulo E0,
the original Tate polynomial and the cubic relation. In particular b,c
have orders3,2 and leading coefficient1. Write A=1+c,
delta=b+c*A^3, Delta=b^3*delta. The actual intermediate tower is

    H(Z)=Z^8+Delta*Z^2+A^2*Delta*Z+C0,
    C0=b^4*(b^2+A^2*b+c*A^4),
    lambda^2+A*lambda=Z+b.

Here Delta=t^11*d and C0=t^14*e by exact polynomial division, with
d(0,1)=e(0,1)=A(0,1)=1. The symbol C0 in this formula is a coefficient,
not the rational base point formerly named C0.

## Two unramified branches

Set Z=t^3*y. The exact transformed equation is

    H(Z)/t^14=t^10*y^8+t^3*d*y^2+A^2*d*y+e=0.

Together with E0 and lambda's quadratic, its two residue points are
(t,q,y,lambda)=(0,1,1,0),(0,1,1,1). The Jacobian determinant in
(q,y,lambda) is1 at both. Thus each is an etale local chart over t and a
regular local curve ring. Its generic inverse y=Z/t^3 identifies it with
the original tower. Each branch has residue F2 and ramification index1.

## Two tame degree7 branches

Use the monomial chart

    t=w^7*x^2,  Z=w^11*x^3.

There is an exact identity

    H(Z)=w^88*x^24*(1+A^2*d*x+w^11*d*x^4+w^10*e*x^4).

All coefficient functions are evaluated at t=w^7*x^2 and the retained q.
Together with E0 and lambda's quadratic this has residue points
(w,q,x,lambda)=(0,1,1,0),(0,1,1,1). Its Jacobian determinant in
(q,x,lambda) is1 at both. The exact rational inverse is

    w=Z^2/t^3,  x=t^11/Z^7.

Thus these regular local charts have the actual source function field;
they are not finite scalar models inferred from a projected factor power.
In each, w is a uniformizer, x is a unit, and ord(t)=7,ord(Z)=11.
They have residue F2 and tame ramification index7 in characteristic2.

## Proper identification and exhaustion

The generic R22 degree8-by2 tower and its original coordinate recoveries
identify these charts with L. The explicit rational inverses show the
identification on their t*Z nonzero opens. Their displayed smooth local
rings have one-dimensional components dominating the base; no isolated
vertical point is substituted for a source branch. They therefore agree
locally with the normalization in L.

The two lambda residues distinguish the branches within each chart. The
ratios ord(Z)/ord(t)=3 and11/7 distinguish the two charts. All four are
rational over F2. Their degree contribution is1+1+7+7=16. Since the proper
map C->E has degree16, the finite-flat/DVR fiber formula leaves no further
branch above O. This is a complete fiber, not a completed global divisor.
The source-normalization and finite-flat/DVR steps are written mathematics;
the Lean map in PROOF_MAP.md states what is formalized.

## Original-coordinate and registered-denominator orders

O24-series-02 uses the four already certified unique smooth-chart lifts,
computes them modulo w^192, and checks both original chart equations and
lambda's quadratic at that precision. It uses the exact original n equation
retained in R22's generic record. Its n coefficient is checked to be
A^2*Delta. All five original source residuals then vanish modulo w^100.
Every reported order has a nonzero leading coefficient1 below100. On the
tame chart, dividing by the order77 denominator leaves at least115 correct
coefficients, sufficient for all the displayed orders. The polynomial tails
above that effective precision are not claimed to be the true nu expansion.

| Point | residue lambda | e | b | c | lambda | nu | delta | S | N | nu numerator | nu denominator |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| O1,0 |0|1|3|2|5|0|2|0|0|11|11|
| O1,1 |1|1|3|2|0|0|2|0|0|11|11|
| O7,0 |0|7|21|14|11|32|14|44|66|109|77|
| O7,1 |1|7|21|14|0|21|14|44|66|98|77|

All four points have degree1 over F2; A is a unit. The complete eight-entry
registered list is exactly nn[1],nn[12],nn[2],nn[2],nn[20],nn[37],nn[65],nn[75].
Their orders at either unramified point are(3,2,2,2,5,2,0,0), and at either
ramified point(21,14,14,14,35,14,44,66). These are polynomial entries from
the original circuit, not a new denominator convention. In particular S,N
are nn[65],nn[75] with original circuit denominators1.

Finite truncation supplies these exact orders because the unit-Jacobian
charts have unique formal lifts and because the source rational-function
identity is fixed. It is not a proof from an arbitrary scalar arc. The
derived O contributions to zero divisors are16 for lambda,53 for nu,88
for S and132 for N (sum the four local orders, since each degree is1).
They do not complete the respective zero divisors elsewhere. No pole of
lambda or nu occurs above O. The four finite rational polar supports,
additional zero supports and the complete prescribed RR space remain open.

## Constant field and geometric integrality

CF24-factor-01 factors the unchanged R22 primitive plane polynomial over
F4[u,lambda], F4=F2[a]/(a^2+a+1). It has one irreducible factor of lambda
degree32 and multiplicity1; the product is exact and the derivative gcd1.
The positive split quadratic and negative irreducible quadratic controls
pass, including evaluation at all four possible roots of the latter.

Let kappa=H^0(C,O_C), the full constant field. The pullback of a degree1
divisor from E has degree16, so [kappa:F2] divides16. R23's degree10 place
also makes it divide10. Thus only degrees1,2 are possible. Nonsplitting over
F4 excludes degree2 in the source-bound field, giving kappa=F2.

The four rational points just constructed give a second written argument:
evaluation injects the field kappa into their residue field F2. Consequently
kappa=F2. The smooth proper curve C is geometrically normal; with this
constant field it is geometrically integral. This resolves the R23
constant-field caveat for this specified incidence field. It does not
identify C with an independent level77 model or supply a modular-factor map.

## Sources

Read the source-bound R22 CM and tower/projection certificates and R23 CM,
scope and descent notes before choosing these charts. Primary references:
[normal proper curves](https://stacks.math.columbia.edu/tag/0BXX),
[etale Jacobian presentations](https://stacks.math.columbia.edu/tag/03PA),
[constant field and degree divisibilities](https://stacks.math.columbia.edu/tag/0AYQ),
[proper geometric normality and integrality](https://stacks.math.columbia.edu/tag/0FD3).
The global constants/scheme argument is written, not a claimed Lean
formalization. No novelty or IC performance claim is made.
