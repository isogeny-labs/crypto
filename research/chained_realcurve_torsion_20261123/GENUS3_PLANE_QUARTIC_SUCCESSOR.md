# Prospective genuine genus-three Jacobian source gate

This written successor addresses the old product-PPAV source gap. No new
plane-quartic census, bitangent calculation, J[4] arithmetic, rational
nontrivial4-isogeny or ECDLP solve has run. All statements are unsigned.
[pcc:unverified]

## Why the source must be genuinely nonhyperelliptic

The old product example uses E/F5:y^2=x^3+2x+1, trace-1 and order7.
A genus3 curve with Weil polynomial (T^2+T+5)^3 would have
N1=9,N2=53,N3=84. A hyperelliptic curve over F25 has at most2*(25+1)=52
points (its genus-zero quotient is P1 over the finite field), so that source
cannot be hyperelliptic. Independently, the8-Weierstrass permutation model
cannot produce the6-dimensional mod2 characteristic polynomial Phi3^3.
Do not upgrade the decomposable E^3 example to a smooth Jacobian by declaration.
[pcc:unverified]

The analogous E/F3 trace-3 cube would predict N1=13,N2=1, contradicting
C(F3) subset C(F9). That source is excluded before an experiment. The F5
expectations above pass this elementary necessary condition; existence is
still a question. [pcc:unverified]

If an actual smooth plane quartic has mod2 Weil polynomial Phi3^3, every
Frobenius-invariant subspace of J[2] has even dimension: all module composition
factors have irreducible degree2. No stable dimension3 Lagrangian exists.
Thus rational multiplication[2] cannot start with a rational (2,2,2) step.
That is a structural scalar example, not a nontrivial DLP improvement.
The old non-scalar mixed4-kernel still requires a genuine polarization/4-torsion
bridge and a target-Jacobian/descent check. [pcc:unverified]

## Fixed bielliptic plane-quartic family to freeze next

Let D(X,Y)=Y*(X^3+2X*Y^2+Y^3), and
g2=a*X^2+b*X*Y+c*Y^2 over F5. For all125 triples and both quadratic square
classes kappa=1,2, consider the250 equations

```text
(kappa*Z^2+g2(X,Y))^2 = D(X,Y).
```

The quotient v=kappa*Z^2+g2 is the known elliptic curve v^2=D, affine
v^2=x^3+2x+1. Its degree2 pullback supplies an actual elliptic-factor route,
rather than an abstract eigenspace. These are250 equations, not250 distinct
isomorphism classes or a complete moduli census. [pcc:unverified]

The fixed binary quartic D is squarefree. The plane curve is smooth exactly
when g2^2-D is squarefree as a binary quartic: a singularity has Z=0 or v=0;
the latter would force D and both derivatives to vanish. For a=0 its affine
polynomial has degree3 with nonzero cubic coefficient, so infinity is simple;
for a nonzero it has degree4. An exact gcd/derivative test suffices, with all
singular cases retained. Smooth plane quartics are genuine nonhyperelliptic
genus3 curves. [pcc:unverified]

A new Rust-only source-first protocol should count all projective points
over F5,F25,F125 for every smooth equation, using direct quartic evaluation
and an independent elliptic-fibre count. Derive the full degree6 Weil
polynomial by Newton identities, require exact division by T^2+T+5, and
record both (i) mod2 Phi3^3 and (ii) full (T^2+T+5)^3 flags. Freeze selection
of the first lexicographic candidate, preserving an empty outcome.
No post-hoc change of source field or search family. [pcc:unverified]

The paired kappa twists give independent controls: odd-degree point counts
sum to twice the elliptic count (N1 sums14, N3 sums224), while N2 is equal.
For every smooth row, mutating N3 by+3 changes the middle Weil coefficient
by1 and must break divisibility by the fixed elliptic polynomial; pristine
counts must pass again. Preserve full rows, source/limits/guards and every
negative result. Native workload needs a separate frozen admission, with
no old source population replay or paid/EKS workaround. [pcc:unverified]

## Further gates, not completed results

If a full E^3 source is found, a written local2-adic route is available:
Z2[pi] is the unramified quadratic order (discriminant-19 is odd, Phi3 is
irreducible), the Tate lattice is free rank3 there, and the principal pairing
gives a unimodular Hermitian form. A Gram--Schmidt/norm-unit argument could
identify its local polarized module with the product model and transport
the old mixed kernel. This needs independent theorem/instance validation;
it is not an executed polarization or quotient construction. Target
indecomposability and the genus3 Serre descent obstruction remain explicit.
[pcc:unverified]

An actual nonhyperelliptic J[2] route uses28 bitangents and their contact
divisors. Pair differences give63 nonzero2-torsion classes; disjoint-pair
equalities can be checked through a conic containing the4 contact divisors.
This is only a lead. Actual J[4]/Weil4 arithmetic may require generic
Riemann--Roch/Khuri-Makdisi methods; no missing engine is fabricated.
The current E7/Richelot qualification and required report/cost work stay
ahead of this new source search. [pcc:unverified]
