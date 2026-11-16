# Complete affine charts from the saved RR spaces

The inverse-base chart has196 checked RR vectors, and rr-pid50-01 PASS gives
the exact rank32 polynomial-module basis and index376. The remaining concrete
tasks there are multiplication-table checks and comparison with the saved
normalP presentation. A22-variable standard-basis cap is retained; do not
repeat that verifier unchanged. Keep the separately prepared14-variable
infinity verifier as an independent presentation-check lane.

## First gluing lead: transport the checked chart itself

There is a more direct source-cover construction to try before rebuilding a
second large RR intersection. Let U0 be the normal affine chart excluding
the A/B fibers, with z=1/u and V=z^2*v. Transport U0 by the actual diamond
T. Function pullback gives U1=T^-1(U0), excluding O/D. The excluded sets
are disjoint, so U0 and U1 cover the proper normal source. They are abstractly
isomorphic affine algebras, with distinct embeddings in the source field.

The overlap has remarkably simple principal-open equations. On U0,

    h0 = u(T(P)) = z+V,
    z1 = 1/h0,
    V1 = (V+zV+z^2)/h0^2.

The inverse base translation is u(T^-1(P))=(u+v+1)/u^2. On the abstract
second copy its removed A/B support is therefore cut out by

    h1 = z1+V1+z1^2.

Using E_inf=V^2+z^2 V+z+z^3, the complete identities are

    h0^2 + z*(1+z+z^2+zV) = E_inf,
    h1^2 + z*(1+z+z^2+z^3+zV) = E_inf

in the appropriate copy (h1=z+V+z^2 in its abstract coordinates).
In particular z is a unit after inverting either h0 or h1. Under the
proposed forward overlap map, h1 maps to
(1+z+z^2+zV)/h0^2=1/z. These identities should be checked as FULL native
products, together with the actual lambda/nu maps and their inverses.

Use the actual F2 diamond formulas already retained from R27:
A=1+c, k=b+A*c, s=c^3/k, ell=k/c,
lambda'=(lambda+ell)/s, nu'=(nu+lambda*b+b*c)/s^3.
The same formulas apply to the descended coordinates: the physical gamma
terms cancel. The field automorphism and its inverse still require binding
to the accepted R50 degree32 field; a base-elliptic identity alone is
insufficient. Generic base inverses are only quadratic-field inverses;
lambda transforms affinely, so its degree does not grow under composition.

For explicit regularity of ALL32 normalized generators, use the saved
originalLift50. If its largest coefficient degree is d, each basis element
has poles bounded by (14+d)D_A+(2+d)D_B. After pullback, h0^(14+d) clears
the O/D poles. Compute each cleared field image, multiply its32 coefficient
vector by the common denominator, and solve for it in the checked PID
basis with a FULL lift. Do the inverse direction with h1 and the inverse
diamond. This produces actual localized-ring maps and verifies exceptional
branches through module membership, rather than declaring their denominators
units. With full inverse map certificates it yields explicit normal-chart
gluing. The132-section route below remains an independent finite-u chart
construction if this transport lane exposes an obstruction.

## A complete RR space for the finite-u chart

Write D_X for the degree16 pullback of the elliptic cusp X. The retained
base orbit is O,A,C,D,B, with T advancing the orbit. A and B lie above
u=0, C and D above u=1, and O above u=infinity. D0=7D_A+D_B.
The exact base action from R27 is

    T(u,v)=((u+v)/u^2,(uv+u+v)/u^3).

The infinity valuation shows T(O)=(0,0)=A; the local AS expansion at A
gives T(A)=(1,0)=C. The remaining regular evaluations give
T(C)=(1,1)=D and T(D)=(0,1)=B. This matches the retained cusp transport;
use the accepted source charts, rather than assigning a cusp label from
a coordinate denominator alone.

R43 already supplies a complete1476-dimensional L(12D0), including all
parent expressions and615-jet values. Under FUNCTION pullback f->f o T
and f->f o T^-1, its pole divisors become respectively

    Dminus = 84D_O+12D_D,
    Dplus  = 84D_C+12D_O.

Their coefficientwise minimum is12D_O, of degree192. Thus

    L(Dminus) intersect L(Dplus) = L(12D_O),
    dim L(12D_O)=192+1-61=132.

The maximum has degree2880, so the sum space has RR dimension2820.
The expected joint rank is1476+1476-132=2820. Evaluation at the same
615-jet degree5 point Q has degree3075>2880, hence is injective on the
whole sum space under the retained proper/pole bridge. No increased
precision or regenerated accepted Newton jets are needed. Read the saved
source jets, apply the actual T and T^-1 rational formulas, evaluate the
saved parent expressions, and compute the132-dimensional intersection
with original-column witnesses and independent recombination. Full field
identities may then be checked from the reconstructed expressions.

Since sigma=T^5 preserves the full original divisor and its complete space,
the T^4 pullback space equals the T^-1 pullback space; this is an available
implementation of the inverse-space transport, not an assertion that
T^4 and T^-1 agree pointwise. The source T formulas descend to F2: the
physical twist shifts nu by b*gamma and lambda by(1+c)*gamma, and these
gamma terms cancel in the descended T updates. Keep both orientations
explicit when evaluating the sections.

All poles of12D_O lie over u=infinity and192>2*61-2+32=152. The complete
RR generation lemma therefore makes these132 sections generators of the
FULL finite-u normalization over F2[u]. Convert them into the actual32
field coordinates, compute the PID module and multiplication table, and
check the overlap against the inverse-base chart. This is the next useful
proper-source construction; it does not require a coefficient-box map search.

## Larger-n continuation

marked23-50-02 certifies the actual degree7 prime base, nonzero Tate
discriminant, inverse coordinates and exact order23 point. For the next QR7
extension use ONLY the three universal source equations nn[239],nn[241],
nn[243] (S3,S4,S5 in R43 source-polynomials.txt). The earlier nn[49] and
nn[53] are the old order11-base conditions and must not be imposed on the
marked23 base. Substitute the accepted b23,c23, reduce coefficients by f23,
and save all actual images before standard-basis work. Compute and export
the generic quotient dimension (prospective112), then all components and
the selected field, with full forward/reverse and denominator certificates.
Do not call a112-dimensional quotient a field before its prime check.

## Separate target obligations

The explicit rank16 sign quotient is useful geometry for reconstructing the
source double cover. R26's character calculation puts the desired CM factor
in the anti-invariant part, so the quotient by itself is not the chosen
factor transfer. Complete the normalized source/pole spaces, then construct
the intended map/correspondence and test nonzero order11 action. The ordinary
reverse-divisor, node187 and target polarized-saturation lanes retain their
existing obligations; this source normalization does not certify them.
