# Post-run connectedness and genus deduction [pcc:unverified]

This deduction combines the frozen branch gate with previously committed chart evidence. It does not change the protocol, add a run, or claim independent review.

The two F61 quartics have separable irreducible specializations at t=4 (map0) and t=11 (map1). Leading coefficients are nonzero there. The primitive quartic is therefore irreducible over F61(t). Primitivity follows because delta4 and the norm root r are coprime, while the odd coefficient -delta1 is a nonzero scalar multiple of r.

The earlier exhaustive chart record contains these affine points:

- map0: t=1, rho=8, b=(55,60,8).
- map1: t=0, rho=39, b=(48,48,39).

Each belongs to a parameter where the specialized quartic is separable (the native chart gate excluded repeated-root projections). Thus the partial derivative with respect to rho is nonzero at the point, so it is a smooth F61-rational point of the irreducible affine curve. This smoothness statement concerns the quartic equation in t,rho, not merely a Veronese point.

An irreducible curve over a finite perfect field with a smooth rational point is geometrically irreducible. To see why, arithmetic irreducibility makes Frobenius act transitively on the geometric irreducible components. A smooth geometric point lies on exactly one component. Its rationality fixes the point, so Frobenius fixes that component; transitivity then permits only one component. The curve is geometrically reduced here because the function extension is separable. Equivalently the degree-one place excludes an enlarged constant field.

Consequently the degree-four geometric cover for each F61 map is connected. Combine this with DERIVATION.md's local sign-triple inertia calculation and the executed genericity checks:

- four distinct finite cubic critical values, each transposition, contribution4;
- three distinct finite paired source-branch values, each double transposition, contribution6;
- infinity with two source-branch preimages among three unramified trigonal preimages, contribution2.

No other branch values occur in the finite sign-triple cover. Characteristic61 is prime to all inertia orders (indeed to the order24 signed-permutation group on complementary triples), so the ramification is tame. For the smooth projective normalization the Riemann-Hurwitz formula gives

    2g-2 = 4*(-2) + 12 = 4,

hence g=3 for both maps.

This is a mathematical deduction under the identification of the scalar chart with the generic Smith sign-triple cover, supported by the rational inverse in native-smith-trace/DERIVATION.md. That derivation and the local inertia analysis are not independently reviewed or formally verified. This document does not claim to compute the normalization or a canonical basis, prove nonhyperellipticity, establish target divisor transport, or measure a speedup. Genus3 alone does not justify using a nonsingular plane-quartic solver.

The p211/p401 maps have no rational delta1 and were not subjected to these rational quartic irreducibility checks. The polynomial square-class identities for them do not remove their transport obstruction or extend this executed connectedness certificate.
