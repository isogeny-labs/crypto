# The explicit two-chart proper model

The accepted candidate, denominator and forward certificates identify the
source field with F2(u)[t]/(m), via lambda=t, v=A/B and
n=(CB+DA)/B. Here t denotes the unscaled field generator. The integral
chart uses the distinct scaled coordinate t_integral=u^4*lambda; its
polynomial F has degree32 and is monic. All scale maps are explicit in the
native inputs rather than identified by name alone.

Let Af=F2[u,t_integral]/(F) and
Ai=F2[z,T]/(F_infinity). CHART50 checks
F_infinity=z^128 F(T/z^4,1/z), integrality and monicity in T.
Both algebras are free of rank32 over their polynomial base rings, with
the power bases through exponent31. CHART50's eight full polynomial
products and two unit maps identify their localizations on u,z invertible:

    u=1/z,  t_integral=T/z^4;
    z=1/u,  T=t_integral/u^4.

The certificates include both defining-equation products and all six
coordinate round trips after adjoining the inverse base parameter. Thus
they glue affine schemes over the usual two affine charts of P1_F2.
The resulting morphism is finite locally free of rank32: finiteness is
local on the target, and the displayed monic power bases supply freeness.
A finite morphism is proper and projective, so this gives an explicit
proper integral model of the source function field. Integrality follows
because each algebra is free, hence torsion-free over its base domain,
and injects into its generic algebra, the certified field. This is a
written scheme construction from the actual polynomial certificates;
Rational50 supplies the field-map and coordinate-roundtrip algebra.

References read: [Stacks01WG](https://stacks.math.columbia.edu/tag/01WG),
Lemmas29.45.3,11,16 for target-local finiteness and finite proper/projective;
[Stacks0307](https://stacks.math.columbia.edu/tag/0307) for localization of
integral closure. Normalizing both charts therefore gives compatible
normal rings on the overlap. The separately frozen native producer/checker
must supply those normal rings and maps. The next divisor calculation
then locates zeros and poles of the original rational coordinate formulas
on these normal charts.

## A prepared intermediate-ring lead

R27 CM and R34 LOCAL_GEOMETRY already identify the degree16 extension of
the elliptic base v²+v+u³+u=0 and the genus31 sign quotient. Those are retained
results, not newly discovered covers. A useful new input representation
would retain the known integral function v before asking for full integral
closure. Test the scaled triangular relation

    A(t_integral/u^4)+B(t_integral/u^4)*v=0

for polynomial coefficients and monic degree16 in t_integral. Together
with the monic quadratic base relation, this would give an explicit rank32
intermediate algebra over F2[u]. Its map to the plane model is finite and
birational because v=A/B in the already certified field, and v is integral
by its base equation. Its normalization is consequently the same integral
closure. This is a structural route to remove known nonnormality before
the generic normalizer; it requires its own native input and complete
checks before use. No normalization run or runtime comparison is recorded
for this intermediate representation here.
