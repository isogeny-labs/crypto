# Level77 normalization,poles and the explicit exceptional support

The proper level77 factor map remains open. This round constructs and
checks a function-field model and complete divisors on its genus-one
base. It does not replace the requested full lifted pole divisor or
complete prescribed Riemann–Roch space by another guessed ansatz.

## Exact Tate11 base

Expanding the unchanged CM17 Tate/QR circuit gives the Tate relation

```
F(b,c)=b*c^7+b^2*c^5+b*c^6+b^2*c^4+c^6
       +b^5+b^4*c+b^3*c^2+b^2*c^3=0.
```

With r=b/c,s=c²/(b+c),this is birational on the original open to
r²+r s³+r s²+s=0. Set u=1/s and
v=(r u³+1)/(u+1). The smooth cubic is

```
v²+v=u³+u,
rn=(u+1)v+1,  cn=rn+u³,
r=rn/u³,  c=cn/u⁴,  b=rn*cn/u⁷.
```

Singular checks both rational inverses and the forward Tate equation;
its adjunction routine reports degree3/genus1. Independent Rust arithmetic
in F2[u,v]/(v²+v=u³+u) also gives exact zero for the Tate substitution.
Lean proves the rational equation and the underlying universal integer
lift. The full Tate-coordinate conversion and divisor semantics have
explicit native/written bridges in the proof map.

Let A0=(0,0),B0=(0,1),C0=(1,0),D0=(1,1),and O be infinity. The symbols
A0 etc below name places;write a1=1+c for the Tate coefficient. With
delta0=b+c(1+c)³,the complete base divisors are:

| Function | A0 | B0 | C0 | D0 | O | Other support |
|---|---:|---:|---:|---:|---:|---|
|b|−7|1|1|2|3|none|
|c|−4|−1|1|2|2|none|
|delta0|−16|−4|8|5|2|+Z5|
|a1=1+c|−4|−1|0|0|0|+SS5|
|K=b+c+c²|−8|−2|3|5|2|none|
|x(4P)=b*u|−6|2|1|2|1|none|
|a1² b³ delta0|−45|−3|11|11|11|+Z5+2SS5|

Z5 is a degree-five closed zero of delta0 above
q5=u⁵+u⁴+u³+u²+1 (hex3d). The exact norm numerator of delta0 is
u¹²(u+1)¹³ q5,with denominator u³². SS5 is the degree-five closed
zero of1+c above qSS=u⁵+u³+1 (hex29). Its norm numerator is u³ qSS,
with denominator u⁸. Native Rabin checks certify both quintics irreducible.
Local series of precision192 give all four finite rational-place orders;
the smooth infinity chart supplies the O orders. The simple residual norm
factors and degree balance certify that the listed supports are complete
on this base. These are not yet divisors on the degree16 level77 cover.

## Actual generic QR model and its relation to the incidence curve

Write Delta=b³ delta0 and a1=1+c. The native generic Groebner computation
has vector-space dimension16 and gives

```
G(lambda)=lambda^16+a1^8*lambda^8+Delta*lambda^4
          +a1^3*Delta*lambda
          +b^4*(b^4+b*c*a1^3+c^2*a1^4).
```

The second triangular equation recovers nu with coefficient a1² Delta.
Both lift matrices are retained;the corrected third certificate invocation
checks both matrix products exactly. The first two attempts retain their
Singular output-type/reserved-name errors. Their process exit0 is not
treated as mathematical success.

Substitution of the genus-one base and quadratic norm in v gives an
irreducible degree32 plane projection over F2(u). The norm identity and
the linear recovery of v have exact zero residuals,and the recovery
coefficient has gcd1 with the plane polynomial. Thus this is a checked
birational function-field model. The original registered affine open is
smooth: saturating its Jacobian singular-locus ideal by every registered
unit gives the unit ideal. The primitive plane model still needs
normalization at its exceptional branches.

The supersingular SS5 support is essential here. At a1=0,nu recovery
has a zero denominator although the original incidence open is smooth.
The lambda polynomial becomes inseparable at that fiber;the original nu
coordinate retains information lost by that projection. A controlled-pole
search must account for these branches and the cancellations in nu.

There is also an exact smaller tower. Put z=lambda²+a1 lambda+b. Then

```
H(z)=z^8+Delta*z^2+a1^2*Delta*z
     +b^4*(b^2+a1^2*b+c*a1^4)=0,
lambda²+a1 lambda=z+b.
```

The native CAS checks G(lambda)=H(lambda²+a1 lambda+b),the compact G
formula and the nu denominator. Lean proves the general characteristic2
linearized decomposition. This supplies a degree8 subfield and a degree2
extension generically. Normalizing the smaller intermediate model before
lifting the quadratic extension is a concrete next route. It does not
remove the a1=0 exceptional fiber or certify a map to the Koblitz factor.

## Resource-limited normalization attempts and the RR gap

All CAS arithmetic uses native Singular4.4.1,with4GiB address space and
1200s per invocation. Four-variable affine closure normalization times
out. Hamburger–Noether places for the degree96 plane exhaust memory.
A registered search of1384 unimodular monomial changes in[-8,8] reduces
the total degree to72,using u↦1/u;its place calculation also exhausts
memory. Those failures are preserved. A separate local integral-basis
calculation on the monic degree32 chart has its own final status in the
validation record. No failed or incomplete run supplies proper places.

Still required: all proper normalized charts and transition maps;the
complete lifted divisors of b,c,lambda,nu and every registered denominator;
a complete basis or certified dimension for a prescribed L(D);and an
actual nonconstant factor map or integral correspondence. Its function-
field identities,descent compatibility and nonzero order11 action must
all be checked. The shared residual Hecke system makes the last test
mandatory. No richer direct relations are claimed without the arbitrary-
divisor reverse evaluator. The ordinary460/367 fibers/images remain open.

The separately registered trace-subfield computation produces an irreducible
monic degree16 plane polynomial over F2(u),of total degree56 after u inversion.
The initial metadata print called the wrong deg overload;its failed output is
retained. The corrected metadata-only check gives v-degree0,total degree56
and exactly the same plane bytes. No normalization job is repeated for that
correction. Adj_div on this smaller plane also exits14 at the4GiB cap before
normalized branches. This records a smaller exact model,but still does not
supply the proper trace places or the quadratic lambda/nu lift. See
runs/cm22-intermediate-01.log and runs/intermediate-metadata/.

## SS5: exact original incidence fiber

This calculation keeps the original nu coordinate and all eight registered
unit entries. Over k=F2[alpha]/(alpha^5+alpha^3+1),put b=alpha+1,c=1.
The source is the unchanged polynomial-ring circuit,not its generic
nu-elimination formula. Native Singular proves equality of the specialized
five original residuals with

```
L=lambda^4+b*lambda+b^2,
C=nu^4+b^3*nu+b^4*(lambda+1).
```

Both explicit ideal-inclusion lift matrices have zero residuals. The quotient
has k-dimension16;every registered unit,lambda and nu is invertible. The
relative lambda,nu Jacobian is [[b,0],[b^4,b^3]],with determinant b^4 nonzero.
Thus the specialized original fiber is reduced and finite etale over k.
This is stronger than factoring the generic lambda projection:at this
support G=L^4 and H=(z^4+b^2*z+b^3)^2,which are nonreduced projections.
The Lean SS module proves those universal characteristic2 identities and
the literal Jacobian determinant;it does not formalize the fiber scheme.

Native q=32 Frobenius on basis lambda^i nu^j,0<=i,j<4 has
rank(F-I)=12,rank(F^2-I)=8 and F^4=I. For a finite reduced k-algebra,the
Frobenius fixed-space dimensions count residue fields and their splitting
under extension. The dimension16,fixed dimensions4 and8,and period4 imply
four residue fields,each of degree4 over k. They are four degree20 closed
points over F2,with16 geometric points over the degree5 base support.
The full matrices,unit ideals,Jacobians and zero products are retained in
cm22-ss5-fiber-02.log. REPLAY_SS5.sh regenerates the exact script from the
original circuit prefix plus SS5.sing and compares its output byte for byte.
The first attempt's missing library helper and matrix/scalar comparison
errors are retained and are not accepted as a successful certificate.

This certifies the original affine fiber. Interpreting it as the entire
proper normalized fiber additionally uses the pure-curve/function-field
bridge,relative unramifiedness and the generic degree16 accounting. That
bridge has not been extracted as a complete proper chart/transition
certificate in this round. No full proper branch table,ramification or
lifted denominator divisor is claimed. At these affine points b,c,lambda,
nu and all registered units are nonzero;the generic nu denominator's
vanishing at a1=0 reflects coordinate elimination,not a pole of the
retained affine nu coordinate. This now supplies a concrete exceptional
local chart to use in the remaining proper-geometry calculation.
