# Hyperellipticity of the natural W1 quotient

**Derived theorem, with geometric descent details requiring independent
review.** Let k=F_q of characteristic two, K=F_(q^7), sigma(z)=z^q,
and p be either irreducible cubic factor of T^7+1 over F_2. In a W1
presentation let gamma1 have exact sigma-order p and gamma2 exact
sigma-order (T+1)p. Set delta_i=(sigma+1)gamma_i and beta=gamma1 gamma2.
The source is E: y^2+xy=x^3+a x^2+b with b=beta^2. Write V_p=ker p(sigma).

For the **natural** genus-seven quotient D of the standard genus-14 Hess
cover, the proposed necessary criterion is

    D geometrically hyperelliptic  =>  b in V_p.

Conversely, every nonzero b in V_p has a canonical symmetric W1
presentation for which the associated natural quotient is hyperelliptic.
This is a coefficient-locus equality over all W1 presentations; it is not
yet a presentation-by-presentation equivalence. It neither classifies other
genus-seven targets nor proves an attack on a subgroup.

## Proof audit

Over an algebraic closure, the character group U of D/P^1_x is generated
over F_2 by the sigma-orbit of `(delta1,delta2)` and has dimension three.
Its two coordinate projections are injective because each delta has exact
sigma-order p. Every nonzero character has the reduced Artin--Schreier
representative `u/x+v*x` with u,v nonzero, so it has two simple poles.
The conductor-discriminant formula gives

    2g(D)-2 = 8*(-2) + 7*(2+2) = 12, hence g(D)=7.

For any nonidentity deck involution, its quotient is a rank-two
Artin--Schreier cover of P^1 with three such characters, giving
`2g-2=4*(-2)+3*4=4`, hence genus three.

If D is hyperelliptic, its unique hyperelliptic involution eta is central,
commutes with the deck group, and descends to an involution alpha of
P^1_x preserving the branch set `{0,infinity}`. Alpha cannot be identity:
then eta is a deck involution with genus-three quotient. A nontrivial
involution of P^1 in characteristic two preserving 0 and infinity must
swap them, hence alpha is `x -> lambda/x`, lambda in k*.

The induced action on reduced simple-pole coefficients is
`(u,v)->(lambda*v,u/lambda)`. It preserves U and commutes with sigma.
As an F_2[sigma]-module U is irreducible of dimension three, so its
endomorphism ring is F_8. An involution in F_8 is the identity in
characteristic two. Thus delta1=lambda*delta2. Since the fixed field of
sigma is k, `gamma2=lambda^(-1)*gamma1+d`, d in k*. Consequently
`beta=lambda^(-1)*gamma1^2+d*gamma1` lies in V_p, and so does b.

Conversely, if a W1 presentation has delta2=c*delta1 for some c in k*,
inversion
`x -> 1/(c*x)` fixes every AS character equation. Put
`t=x+1/(c*x)`: the invariant field is a rank-three AS extension of
`k(t)` whose seven nonzero character equations have a single simple
pole. The conductor formula gives `2g-2=8*(-2)+7*2=-2`; its genus is
zero. Thus inversion is the hyperelliptic involution.

Existence of the converse's coefficient relation can be seen directly: every
nonzero b in V_p has a unique gamma in V_p with
`b=[gamma*(gamma+1)]^2`, because `gamma -> gamma^2+gamma` is an
F_2-linear automorphism of V_p. Taking gamma1=gamma and gamma2=gamma+1
gives c=d=1. Other W1 presentations of the same b need not share this
specific inversion. A universal presentation-independent converse
requires a separate proof.

## Quantitative consequence

The classical locus has exactly `2(q^3-1)` nonzero b values across both
cubics, density `2(q^3-1)/(q^7-1)`. Any nonclassical W1 coefficient
outside those cubic kernels fails the natural hyperelliptic-quotient test.
The published W1 family remains eligible for its genus-14 cover. This
theorem does not bound all possible covers, isogenies, or correspondences.
