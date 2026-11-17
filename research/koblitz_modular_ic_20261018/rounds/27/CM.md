# Complete original divisors and the Prym cusp module

All claims below separate native identities from their written smooth proper
normalization/DVR/Picard interpretations. No actual desired modular-factor map,
nonzero target image,IC cost or relation-rank claim is supplied.

## Exact transported valuations

Let P[e,j,i]=T^i P[e,j,0], with e=1,7 and j=0,1 the R24 infinity charts.
The cusp order is O,A,C,D,B for i=0,...,4. The accepted LAURENT02 calculation
uses768 seed precision and propagates ABSOLUTE precision through every
Laurent operation. Every original source residual vanishes to positive
certified precision. Fifth iterates agree with sigma, base b/c orders agree
with their complete divisors. Leading coefficients below are all1.

| e,j | function | O | A | C | D | B |
|---|---|---:|---:|---:|---:|---:|
|1,0|lambda|5|-4|2|4|2|
|1,1|lambda|0|-3|0|0|-1|
|7,0|lambda|11|-28|11|11|4|
|7,1|lambda|0|-21|0|0|-7|
|1,0|nu|0|-12|0|0|-3|
|1,1|nu|0|-12|0|0|-3|
|7,0|nu|32|-51|14|25|12|
|7,1|nu|21|-51|7|14|7|
|1,0 and1,1|S|0|-24|0|0|-6|
|7,0 and7,1|S|44|-124|44|44|2|
|1,0 and1,1|N|0|-36|0|0|-9|
|7,0 and7,1|N|66|-186|66|66|3|

The corrupted actual lambda update is rejected by original residual3 and
the fifth-iterate detector despite native exit0/late marker. LAURENT01's
identifier collision is separately retained, never counted as an accept.

## Remaining support and complete divisors

Original lambda is integral at every noncuspidal place because G(lambda)
is monic with regular base coefficients. Its generic nu recovery denominator
has noncuspidal zeros only at SS5/Z5. Outside those the original nu coordinate
is regular. R22/R23 give complete regular fibers at SS5/Z5 and prove nu is a
unit there. Hence both coordinates, and their polynomial entries S,N, have
no noncuspidal poles. This uses the retained field/proper-normalization bridge,
not a claim that the singular plane projection is already normal.

Native NORM01 checks pole degrees lambda64,nu132,S308,N462. Nu's cusp zeros
already total132. S's cusp zeros plus order4 at the degree10 Z7 point total308;
N's cusp zeros plus order6 there total462. Principal divisor degree balance
therefore excludes all other zeros of nu,S,N. The SS5 units and Z1 units
remain as previously checked. Thus their displayed divisors are COMPLETE.

Lambda's cusp zeros total50. The missing support comes from
mu=b^4+bc(1+c)^3+c^2(1+c)^4, the constant coefficient G(0)/b^4.
Its cleared elliptic norm is

    Norm(u^28 mu)=u^22 (u+1)^16 q14,
    q14=u^14+u^13+u^9+u^5+u^4+u^3+1.

Native factorization gives irreducible q14 with exponent1 and checks the
factor product. LAMBDA14-01 works in F2[a]/q14 and finds the single base
residue and an original source point:

    u=a,
    v=a^12+a^11+a^8+a^7+a^3+a+1,
    b=a^8+a^6+a^2+a+1,
    c=a^12+a^11+a^9+a^5+a^2+a+1,
    lambda=0,
    nu=a^11+a^10+a^8+a^7+a^6+a^4+1.

All five original source residuals vanish; all eight registered entries,
nu and the generic recovery denominators are nonzero. Also u=(b+c)/c^2,
so the source residue has degree14. The lambda derivative is nonzero.
The simple norm factor gives a simple lambda zero at this degree14 point.
Its14degrees plus50cuspzeros exhaust64: lambda's divisor is COMPLETE too.
Together with R26's complete base-derived entries this completes all four
original coordinate divisors and all eight registered denominator divisors.

## Proper quotient and an independent genus check

On the registered open b is a unit and sigma(nu)-nu=b; the native fixed-locus
certificate1=ib*b+(b*ib+1) excludes fixed geometric points. At the only missing
noncuspidal registered boundary Z5, the R23 source coordinates are regular and
b is a unit, so the same geometric fixed-point contradiction holds. SS5 is
already in the registered open. At O sigma swaps the two lambda residues0/1;
T commutes with sigma=T^5 and transports this free action to every cusp.
The source is covered by these opens/charts: no additional poles or registered
zeros remain after the complete divisor calculation above.

Thus on the proper smooth normalization the constant Z/2 action is free.
Its quotient is a finite etale double cover, including in characteristic2.
This is a written scheme bridge from the actual charts, using Stacks07S7
and descent of etaleness for a torsor; no classical prime-to-characteristic
Prym polarization theorem is assumed. The proper quotient is the field K
already identified in R25, not a newly identified independent modular model.

There is also a genus check from the actual ramification, avoiding modular
model identification. The degree16 source-to-E map is etale off the cusps,
Z5 and SS5, since the triangular derivative and recovery coefficient are
units there. SS5 is etale by its original relative Jacobian. Each cusp has
two e7 rational points (different degree12); the Z5 ramified point has e7
and degree10 (different degree60). All are tame. Total different degree is
5*12+60=120. Riemann-Hurwitz over genus-one E gives genus61. The etale
double quotient consequently has genus31 and Prym dimension30. Its cusp
fibers have two rational points, e1 ande7. The Z5 quotient has two degree5
points,e1 ande7: sigma acts as the nontrivial quadratic residue automorphism
on each R23 degree10 point. Equivalently its different degree is30+30=60.
These are geometric deductions from chart coverage; Lean only proves the
conditional numerical genus formulas, not smooth curve or RH theorems.

## A concrete norm-one relation and its exact residual module

All eight registered functions are invariant, so applying1-sigma to their
principal divisors gives zero. Native NORM01 checks all eight actual ratios
f/sigma(f)=1. This explains why those functions alone cannot supply nonzero
Prym divisor columns. They remain useful for charts and controlled poles.

The actual ratios gL=lambda/(lambda+A) and gN=nu/(nu+b) are nonconstant
and have norm1. Their conjugates are their inverses. Native exact Hilbert90
also recovers g=(1+g)/(1+g^-1) for each. In characteristic2 this is distinct
from a minus eigenspace of functions.

Let delta_i=P[7,0,i]-P[7,1,i]. The COMPLETE divisor of gN is

    div(gN)=11delta_0+7delta_2+11delta_3+5delta_4.

It has pole degree34 and no noncuspidal support. T shifts delta_i and sends
delta_4 to-delta_0. Thus the five transported principal relations give the
actual negacyclic presentation r(T)=11+7T^2+11T^3+5T^4,T^5+1=0.
Native CUSP01 computes determinant1477212 and the entire mod11 ideal:

    gcd(T^5+1,r)=T+5,
    (2T^2+T+5)(T^5+1)+(4T^3+2T^2-5)r=T+5  (mod11).

The full polynomial lift is checked, rather than only evaluations. The other
primitive values2,7,8 give nonzero r;6 gives zero. Therefore this presentation
has one-dimensional mod11 quotient, with T acting by6. The actual subgroup
of the Jacobian generated by these ramified cusp differences is a QUOTIENT
of this presentation and may be smaller or zero. No nonzero class,actual
modular-factor inclusion or final reverse evaluator follows from the index.
The surviving value6 agrees with the retained residual diamond character;
that agreement still requires a nonzero-image test and is not the F2 map.

## RR work and remaining leads

Use D=7D_A+D_B,degree128. The complete ramification genus check gives the
same dimension68 as R26's retained genus bridge. The first RR coefficient
families observed rank19 atK8,12,16; their fixed O-pole restriction makes
them incomplete. The native marker was reached but a live launcher edit
lost the exit status, so this is a retained wrapper failure, not an accepted
run. RR-O expands the actual coefficient pole range and must satisfy every
20-cusp constraint and129-jet independence check before completeness.

An actual F2 factor map and nonzero11action, arbitrary collected-divisor
reverse evaluation including ordinary460/367, node187 original-ideal
membership, full coupled quotient saturation and proper polarized embedding
remain separate requirements. Subsequent results belong in fresh sections.

Sources: R22-R26 notes/native files above;
https://stacks.math.columbia.edu/tag/0BXX (proper normal curves/function fields),
https://stacks.math.columbia.edu/tag/07S7 (free finite-group quotient torsor),
https://stacks.math.columbia.edu/tag/0C1B (RH and different,including wild caveat),
Sutherland18.782 lecture22 Cor22.23 (RR). No novelty or speed claim.

## Completed RR space, actual class and proper projective equations

The initial restricted coefficient families above are superseded by RR-O01
and complete RR-export01: K16,1568candidate monomials, all20cusp constraints,
68independent valid sections. Original-column129jet independence and full
recombination certify a basis of L(7D_A+D_B), degree128. Each basis function
is an explicit polynomial in nu,lambda,v,u divided by u16(u+1)16.
ACTION01 reconstructs all68functions onall20charts, not only saved vectors.

Singular's bulk list writer omitted the last element: IO01 isolates that
serialization failure. The accepted replay writes each element separately:
20charts,68basisvectors,1568constraints/jetcolumns. Earlier bulk exports are
incomplete and are never used for completeness. Inputs and exports are pinned.

For actual principal relation matrix M, z=(1,2,4,8,5) gives
M z=11d, d=(-9,-11,-4,8,10). Let D0=sum d_i delta_i. Moving4D0 by actual
principal relations and-div(lambda) gives D' with coefficientvector
(1,-3,10,2,0) ondelta and-div(lambda). Its positive/negative parts have degree64
and D'<=7D_A+D_B. The complete68dimensional membership system includes the
simple closed degree14lambda zero; it has rank68/kernel0, all pivots checked.
Thus L(D')=0,D' nonprincipal,11D' principal. D' has exactorder11;[D0]=3[D']
andT[D0]=6[D0]. This supersedes the earlier possible-zero presentation caveat
for the11primary cusp subgroup ONLY. Shared mod11 CM systems still prevent
identifying the desired CM-sign projection from these data alone.

ACTION01 computes the actual68x68involution,checks sigma2=I andrank(sigma+I)=34.
Its34invariant sections form completeL(D_H) onH,degD_H64. QUADRICS03 computes
all2346source and595quotient products, exactranks196/98,kernels2150/497;
everykernel andpivot isindependently recombined from originalproducts. Source
257Ojets force globalzero by poledegree256;quotient129Ojets vanish atboth O
and sigmaO,forcingglobalzero. Actual(b+c)/u+c2=0 and its constant-square
corruption give0/1controls.

Finite-field quadratic-generation Proposition11 of Marquez-Corbella,
Martinez-Moro andPellikaan applies since128>=2*61+2 and64>=2*31+2. Therefore
these COMPLETE quadrics generate the prime saturated homogeneous ideals of
the complete-linear-system closed embeddings ofC inP67 andH inP33. The
recovered1,b,c,lambda generate the original source functionfield (nu recovery
uses the registered rational identity); the H model is its fixed-field
quotient. This is a proof using RR/normal-curve bridges and the cited theorem,
not a giant Groebner saturation computation or equality with oldcoupled ideals.
Reference checked in full, especially the finite-field hypothesis:
https://ruudp.win.tue.nl/paper/65.pdf

## Polarized carrier and reverse controls

POLARIZATION.md gives the written propermixedPrym24 construction from the
free cyclic10tower. Its induced polarization type is(1^18,5^6),overF2,with
explicitintegralcorrespondenceE=(1-T5)(5-N5), E2=10E. Both properPicard and
polarization arguments remain written; no theta-section basis is claimed.
The actual order11class lies in this mixed carrier; its desired CM-sign
projection remains unknown. This is distinct from the old determinant19
polarizedquotient and does not close its embedding obligations.

REVERSE02 supplies an integral matrix W with
 M W=134292 I-d(1,6,36,216,1296).
It evaluates arbitrary ramified-cusp divisor combinations after a prime-to11
scalar; it is not restricted to forward images. The actual nonzero11class
shows this cusp11primary quotient is exact. Ordinary collector classes,
460/367sourceC5lifts and fullreverse divisor evaluation remain separate.

NODE01 verifies80actual Miller functions acrossall4graphs/5conjugates, their
proper-chart pole19units and exact group-law chart lifts forbothordinary
curves. The explicitnilpotentunitcertificate strengthens the old valuative
argument but still is not an h-power membership certificate in the original
homogeneous source ring. Fullcoupled quotient saturation remains open.

LOWGENUS01 confirms both A5Weil polynomials irreducibledegree8 and not even.
Under retained level77modular decomposition, all genus7quotient constituents
are too small; the extra4dimensional quadratic-character orbit in genus13
has evenF2Euler polynomial because chi(2)=-1. Neither A5sign can occur in
those particular quotientJacobians. Otherlow-genus carriers remain open.
Sources: https://www.lmfdb.org/ModularForm/GL2/Q/holomorphic/77/2/a/ and
https://www.lmfdb.org/ModularForm/GL2/Q/holomorphic/77/2/b/ .

The simple trace-one coordinates have degrees34,80,110,which cannot equal
3deg(phi) for elliptic y/x. MAPBUILD02 goes beyond those constant-shift
exclusions: actual34invariants/tracepreimages produce455F32equations for
ALL equivariantX,Y in the completedRRspace, includingnonconstant invariant
adjustments,bothordinarysigns and a nonconstantXblock. Its independent
source-jet controls and capped solve are recorded separately; no evaluated
F2modular-factor map follows from constructing these equations.
