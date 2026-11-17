# Exceptional lifted geometry of the original Tate/QR incidence curve

This is a written geometric derivation supported by exact native certificates.
The complete scheme/normalization/DVR bridge is not formalized in Lean;the
precise formal subclaims and hypotheses are in PROOF_MAP.md. This constructs
local geometry of the specified incidence function field,not an evaluated
modular-factor map or a measured IC result.

## Fixed source and the proper curve

Use the unchanged R22 Tate base E:v²+v=u³+u and its functions b,c. Write
A=1+c,delta=b+c*A³,Delta=b³*delta. The R22 generic QR computation identifies
a degree16 field L over F2(E),with original coordinates lambda,nu. Its
birational plane norm has degree32 over F2(u);the original generic ideal
lifts and linear coordinate recovery distinguish the actual field from a
mere characteristic-polynomial coincidence. Let C be the smooth proper
curve with field L,and pi:C->E its finite map of degree16.

The source closure used here is the saved saturation of the original five
residual numerators by all8 registered unit entries. Interpret its points on
the R22 generic incidence component,which is the closure of the registered
open identified with L. This source/component identification is an input
from R22's source-bound generic/lift calculations;it is not replaced by an
independent modular model. On the registered open away from A=0,the monic
lambda equation and the linear nu equation with coefficient A²Delta give
the finite generic presentation. The only missing denominator support on
that open is SS5;the chart below checks that support in the original nu
coordinate. Thus the original component has no additional vertical branch
there. Saturation takes its closure at the removed discriminant support Z5.

For normal proper curves the map is finite and flat. Its fiber over a closed
base point has dimension16 over the base residue field,with contribution
[e(P/Q) times f(P/Q)] from a normalized branch P. These facts,not a projected
polynomial's factor multiplicity,justify the exhaustion steps below.
Primary references checked:Stacks53.2,https://stacks.math.columbia.edu/tag/0BXX;
59.26,https://stacks.math.columbia.edu/tag/03PA;33.44.9 and33.44.11,
https://stacks.math.columbia.edu/tag/0AYQ. The DVR length calculation is written
explicitly below. No global normalization routine is asserted to have finished.

## SS5: original local presentation and complete normalized fiber

Over k=F2[a]/(a⁵+a³+1),the base point has b=a+1,c=1. R22's retained two-sided
ideal lifts identify the original specialized fiber with

L0=lambda⁴+b*lambda+b²,
C0=nu⁴+b³*nu+b⁴*(lambda+1).

It is reduced of k-dimension16. Its Frobenius certificate gives four residue
fields of degree4 over k. All8 original registered unit entries,lambda and nu
are invertible. These R22 artifacts were restored and their314 raw checksums
verified;they were not recomputed under a different model.

Write F for the exact Tate polynomial,Q0=nn[239],Q1=nn[241],Q2=nn[243].
The saved SS23-certificate.sing contains an actual polynomial h and three
polynomials H_F,H_0,H_1. The fresh compact native replay verifies

h*Q2=H_F*F+H_0*Q0+H_1*Q1.

It also checks nn[49]=cofactorA*F,nn[53]=cofactorB*F,with cofactorA=1 on the
entire specialized fiber. On that fiber h specializes exactly to

U=nu²+b*nu+b*lambda³+b²*lambda.

For B=a⁴+a³+a²+a+1,put
V=(a⁴+a)*nu²+B*nu+B*lambda³+a³*lambda+a³.
There is the exact polynomial identity over k

a³*lambda²*L0+(a⁴+a)*C0+V*U=1.

Thus h is a unit in every source local ring above SS5. The original five
residuals locally generate exactly (F,Q0,Q1). This is an original source
neighborhood,not merely an etale replacement of its reduced special fiber.
The first-host complete colon calculation supplied h;its full raw quotient
output was lost in the second migration. The literal h/lift coefficients
survived in Git,and fresh native and Lean checks certify the required identity
without relying on that missing output.

The original derivatives satisfy,on this fiber,

F_b=b²*(b+1)²,
det(d(Q0,Q1)/d(nu,lambda))=b⁴*U.

Their product is a unit,b⁶*(b+1)²*U. This supplies the standard etale local
presentation over the Tate base. Consequently these16 geometric affine
points are unramified points of C. Their residue-degree contribution is
4*4=16,so the proper degree16 fiber has no additional branch at infinity.
Over F2 the complete fiber has four closed points,each of degree20,e=1.

At every one,b,c,lambda,nu and every registered unit have valuation0.
The base function A has its simple SS5 zero,so its lifted valuation is1.
The generic nu denominator D=A²*b³*delta has valuation2. Since the retained
nu coordinate is a unit,its generic recovery numerator has valuation2 too.
This records an exact cancellation at the primitive projection's denominator;
it does not replace nu by the inseparable lambda projection G=L0⁴.

## Z5: the original thickening,regular closure and tame ramification

Over k=F2[a]/(a⁵+a⁴+a³+a²+1),the unique base point selected by the actual
elliptic equation and delta=0 has u=a,v=a⁴+a,b=a⁴,c=a⁴+a³+1. Both base
residuals vanish exactly. The original unsaturated fiber has length20;
the saved saturated closure has length16. Complete saturation by the actual
seventh unit entry and complementary ideal quotient give comaximal splits

original:2+18,       closure:2+14.

Both closure residue fields have degree2 over k. Their common irreducible
lambda polynomial is

q=lambda²+(a⁴+a³)*lambda+(a²+1).

The two residues are

P1:nu+(a⁴+a²+1)*lambda+(a³+a²)=0,
P7:nu+(a⁴+a²+1)*lambda+a=0.

The first closure component is already that field. The second is a curvilinear
primary algebra with lambda eliminant q⁷ and nu a displayed polynomial in
lambda. Its full Groebner basis is retained. Exact intersection and
comaximality checks certify the decomposition;the original q⁸ projection
and length18 thickening are not substituted for it.

Differentiate every one of the51 saved closure generators in the original
four-variable ring before specializing. The minor with rows8(F),1,2 and
columns(lambda,b,c) reduces respectively to

(a⁴+a+1)*lambda+(a⁴+a³+a+1),        a⁴+a³+1.

Each generates the unit ideal with its actual residue ideal. Hence the
local rings on the source curve are regular;over the perfect finite field
they are smooth curve points and agree with the normalization locally.
This is the source-regularity check that was missing from the scalar and
projected-fiber descriptions.

The base zero delta is simple. In a regular local curve ring with uniformizer
t,delta is a unit times t^e. Its quotient has length e over the residue field.
The k-dimensions2 and14,with residue degree2,give e=1 and e=7. Their weighted
sum is2+14=16,so these exhaust the proper fiber. Over F2 there are two closed
points of degree10. The ramification index7 is tame in characteristic2.

Write S=s1*s2+s3,N=b³+A*b²*s1+A²*b*s2+A³*s3,where
s1=b+lambda²+A*lambda,s2=A*nu+b*lambda,s3=nu²+b*nu.
These are the actual seventh and eighth registered polynomial unit entries;
the unchanged circuit gives their denominators1. At P1 both are units.
At P7 the native quotient dimensions are8 for(delta,S) and12 for(delta,N).
In the DVR these equal2*min(7,v(S)) and2*min(7,v(N)). Therefore v(S)=4 and
v(N)=6. This uses source regularity;it is not inferred solely from q⁷.
Both lambda and nu are units at both residues. The other registered entries
b,c,c,nn[20],nn[37] are also units. D=A²*b³*delta has valuations1 and7,and its
nu-recovery numerator has exactly the same valuations because nu is a unit.

## Lifted orders now fixed on these two exceptional supports

| Closed points over F2 | count | degree each | e | b,c,lambda,nu | A | delta | S | N | D |
|---|---:|---:|---:|---|---:|---:|---:|---:|---:|
| SS5 |4|20|1|0,0,0,0|1|0|0|0|2|
| Z5 P1 |1|10|1|0,0,0,0|0|1|0|0|1|
| Z5 P7 |1|10|7|0,0,0,0|0|7|4|6|7|

These are complete over SS5 and Z5. They are not complete global divisors.
The five rational base supports A0,B0,C0,D0,O still require their normalized
lifted charts,transition maps and orders. A prescribed complete RR basis,
an actual nonconstant modular-factor map/correspondence,descent compatibility
and nonzero order11 action remain required. Node187 source-bound polynomial
membership,full quotient saturation/proper polarized embedding and an arbitrary
modular-divisor reverse evaluator with ordinary460/367 fibers/images also remain
separate open requirements. No new relation rank or cost is reported here.
