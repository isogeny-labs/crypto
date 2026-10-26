# Prepared three-torsion does not guarantee a usable quotient

The following example refutes the stronger coverage claim that **every**
prepared scalar-minus-I curve has at least one unobstructed member among its
12 transverse rational (3,3,3)-kernels. It does not refute the weaker existential
claim allowing one freely chosen additional isogeny even for an already
prepared input.

Let

    K = F_109[z]/(z^3+z+103),
    E: y^2 = x^3 + (37z^2+105z+29)x + (43z^2+6z+66).

Then

    #E(K) = r = 1293169, prime;
    109 = 1 mod 3; r = 4 mod 9;
    trace(E/K) = 1861, not divisible by 109.

The j-invariant is not in F_109. The third division polynomial splits into four
linear factors. Since the rational group has prime order different from three,
Frobenius acts as -I on E[3]. The ninth division polynomial also has all 40
x-coordinate roots in K. This is a feature of this example, not a proposed
sufficient obstruction criterion.

An explicit source-order certificate uses

    P = (12z^2+28z+3, 78z^2+59z+37).

P is nonzero and rP=0. The Hasse interval [1292755,1297305] contains exactly
one multiple of the proven prime r, so the group order is r independently of
the original point-counting call.

## All twelve quotients are obstructed

All 12 candidate quotients reconstruct to smooth genus-three plane quartics
over F_109. All 28 computed bitangents verify on every curve. For every target,

    #C(F_109) = 110,
    #C(F_(109^2)) = 11882,
    #Jac(C)(F_109) = 1296891 = 3^7 * 593.

This is the quadratic-twist order

    2*(109^3+1)-r,

not the order r of the quotient abelian variety. In particular these base-field
Jacobian groups cannot contain the source r-subgroup.

The first two point counts force L(T)=1+a3*T^3+109^3*T^6. Weil bounds put the
Jacobian order in [1292755,1297305]. Exact rational-point divisor orders then
leave a unique possible multiple in that interval. Explicit base points,
points, coordinate changes, and exact-order checks for every target are saved
in [the witness certificates](results/explicit_counterexample_witnesses.json).
The [full audit](results/counterexample_audit.json) includes all 12 equations,
the original order certificates, and the source and repair witnesses.

Thus this conclusion does not rely only on the chi_18 prediction.

## A single additional 3-isogeny repairs this example

All four immediate rational 3-isogeny neighbors remain torsion-eligible. Their
chi_18 screens find respectively 5,3,4,5 unobstructed candidates.

One explicit repair has kernel polynomial

    x + 94z^2+107z+52

and codomain

    E': y^2 = x^3 + (14z^2+58z+33)x + (75z^2+73z+66).

The isogeny preserves the prime-order subgroup. A resulting smooth quartic's
Jacobian order is independently certified as r=1293169. Its equation and
subgroup-image witness are in the audit.

The original bad curve has no immediate nonscalar-three-torsion neighbor.
Therefore this example does not supply a source with a unique forced
preprocessing step into the bad model. It disproves the policy of stopping
as soon as the torsion screen passes, while leaving the weaker existential
one-step conjecture open.

The concrete pipeline should therefore check the actual obstruction and move
to another isogenous model when all twelve candidates fail. No uniform bound
on the number of additional steps is established here.

## Search provenance and novelty boundary

The counterexample was found as the 4430th accepted prepared Frobenius-orbit
representative in a separate Hesse-family search. The search stops at the first
all-negative hit and independently certifies that hit. It is not an unbiased
density estimate. Earlier runs covered 1000 representatives in eight new
characteristics and closed 49 classes in three targeted 3-isogeny components.

This is a counterexample to the conjectural sufficient condition developed in
this conversation. It is not asserted to refute a theorem of Tian or to be the
first such example in the literature. Isogeny preprocessing, Serre's
obstruction, and quotient construction have established prior art.

Reproduce the audit and explicit witnesses with SageMath 10.6:

```sh
sage -python research_lead/counterexample_audit.py
sage -python research_lead/explicit_counterexample_witnesses.py
```

The saved [search records](results/fast_counterexample.json) include the Hesse
parameter, tested obstruction values, and the independent certificates.
