# Exact contract and geometric transfer eligibility

All arguments below are written derivations, without a compiled Lean theorem,
external validator or signed PCC verdict. The native census tests their finite
consequences, not their universal geometric premises. [pcc:unverified]

## Frobenius and exact extension orders

For a squarefree degree5/6 hyperelliptic model in odd characteristic, its six
Weierstrass points give J[2] as even subsets modulo complementation. A pair
of irreducible cubics has root permutation(3)(3), of order3, and characteristic
polynomial (T^2+T+1)^2 on this four-dimensional F2 module. An irreducible
quintic plus the rational point at infinity has root permutation(5)(1), order5,
and characteristic polynomial Phi5=T^4+T^3+T^2+T+1. These conclusions concern
actual curves, using the standard Weierstrass-subset identification; no
abstract companion is substituted for genuine J[8] coordinates.
[pcc:unverified]

If alpha_i are the four eigenvalues, N1=4-s1 and N2=10-s2. Their symplectic
pairing multiplier3 gives e3=3e1=3t,e4=9, yielding P(T) above. For qn=3^n,
e3(alpha_i^n)=qn*s_n and e4=qn^2. Hence
#J(F3^n)=1-(1+qn)*s_n+(s_n^2-s_2n)/2+qn^2.
Newton recurrences and det(I-F^n) are distinct integer algorithms for this
same expression; neither computes Jacobian divisor arithmetic.
[pcc:unverified]

The real quadratic Weil coordinates beta_i satisfy
beta1+beta2=t,beta1*beta2=b-6. With D=t^2-4b+24, requiring both beta_i in
[-2sqrt3,2sqrt3] is equivalent to D>=0,t^2<=48,L=24+4b>=0 and
L^2>=192t^2. Indeed these imply L>=2t^2 and thus
(L-2t^2)^2>=4t^2D, equivalent to |t|+sqrtD<=4sqrt3.
[pcc:unverified]

The kernel of GL4(Z/8)->GL4(F2) has exponent dividing4: for any B,
(I+2B)^2=I+4(B+B^2) mod8, whose square isI. Therefore Frobenius acts trivially
on J[8] after12 or20 powers in the respective sectors; J[8] is rational over
the indicated extension and has4096 elements. If that extension order has
v2=12, its entire2-primary subgroup equals J[8]. This establishes a useful
fixture constraint; basis construction, coordinates and a pairing still need
actual arithmetic. [pcc:unverified]

For every n, the2-primary subgroup H=J(F3^n)[2-infinity] is stable under the
original F3-Frobenius. The groups (2^(j-1)H)[2] are stable subspaces of J[2].
The3+3 module has no fixed line and, by the nondegenerate multiplier1 mod2
pairing, no stable hyperplane; layer dimensions are0,2,4, so v2|H| is even.
The5+1 module is irreducible, forcing all layers to have dimension0 or4 and
v2|H| divisible by4. Unlike a maximal-isotropic-kernel argument, this order
argument does not assume H is isotropic. [pcc:unverified]

## Restriction on elliptic-to-Jacobian transfers over the base field

Phi5 is irreducible over F2 (the order of2 modulo5 is4). A monic integral
Frobenius polynomial with this reduction is irreducible over Q2, by the
monic/Gauss reduction argument over Z2. If a nonzero F3-defined homomorphism
E->J existed, its image would be a one-dimensional elliptic subvariety.
The induced injection on rational2-adic Tate modules supplies a
two-dimensional Frobenius-stable subspace, contradicting irreducibility.
Consequently Hom_F3(E,J)=0 for every elliptic E in this5+1 sector. The same
restriction holds for any F3-isogenous codomain because isogeny preserves
the rational Tate module. No Tate isogeny theorem converse is needed for
this necessary-condition argument. [pcc:unverified]

This excludes such geometric algebraic-group transfer maps over F3. It does
not exclude arbitrary finite-group or computational reductions, transfers
defined over extensions, or general ECDLP algorithms. An irreducible5+1
torsion sector can remain useful for structural negative tests while failing
this particular same-field elliptic transfer eligibility condition.
[pcc:unverified]

The3+3 sector can have either elliptic factors or none. Factorization as
(T^2-t1*T+3)(T^2-t2*T+3) is equivalent to integral roots of
X^2-tX+(b-6) in the elliptic Weil interval. The census records this exact
test and ordinary status. For an ordinary surface, absence of elliptic Weil
factors excludes an F3-defined elliptic subvariety, without claiming absolute
simplicity. A split Weil polynomial alone does not provide an explicit map,
a prime-order subgroup, an efficient inverse or an ECDLP advantage.
[pcc:unverified]

## Sources checked and boundary from previous work

Knowledge-base tools are absent; the repository wrapper enters the locked
crypto-autoresearcher layout and is not invoked. No novelty/absence claim
is made. Pinned crypto816f9c3a sources reviewed in the preceding checkpoint:
src/prime_hyperelliptic/{cover_transfer,normalized_cover_transfer}.rs and
src/cryptanalysis/chained_transfer/bielliptic.rs already supply explicit
elliptic/genus-two cover maps. This census does not rediscover those maps.
Existing actual divisor and prime-field polynomial arithmetic will be
reviewed before a coordinate extension. [pcc:unverified]

Earlier primary-source audit: Smith, arXiv:0806.2995v2 (especially Section9),
Bruin--Doerksen arXiv:0902.3480v3 and the Magma Richelot handbook. Their
geometric, product-endpoint, extension and cost qualifications remain in
force. Generic Jacobian index-calculus exponents must be compared with the
original elliptic subgroup size, not the entire destination Jacobian order.
No end-to-end cost improvement is asserted here. [pcc:unverified]
