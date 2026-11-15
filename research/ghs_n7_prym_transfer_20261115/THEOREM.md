# Exact classical GHS slice and a dihedral factorization

Status: **prior-work collision confirmed**. The symmetric W1 factorization below is an alternative description of the published classical genus-seven GHS slice. Its Prym construction supplies an explicit correspondence and an independent map check, but no new eligible family or smaller attack target.

## Exact coefficient criterion and count

Let q=2^ell, k=F_q, K=F_(q^7), sigma(z)=z^q, and p1(T)=T^3+T+1, p2(T)=T^3+T^2+1. Put V_i=ker p_i(sigma). By the normal basis theorem and T^7+1=(T+1)p1(T)p2(T), each V_i has k-dimension three and V_1 intersects V_2 only at zero.

For every nonzero b in V_1 or V_2, and every trace-one twist a, the ordinary curve E:y²+xy=x³+a x²+b has **classical GHS genus seven**. The exact coefficient criterion is

    b != 0 and [p1(sigma)(b)=0 or p2(sigma)(b)=0].

It contains exactly 2(q³−1) nonzero b values. These are coefficient values for the fixed trace-one isomorphism class; Frobenius orbits identify some over k. The density among all nonzero coefficients is 2(q³−1)/(q⁷−1), asymptotic to 2/q⁴.

The previous W1 second-moment bound also gives a rigorous supply of
**nonclassical W1 coefficients** in each cubic arm. Set
`m=q²+q+1`, `M=(q³−1)²`, and
`D=M+m(m−1)(q²−1)`. If `W1_i` is that arm's published
genus-fourteen eligibility set, then

    ceil(M²/D) − (q³−1) <= #(W1_i \ V_i) <= q⁶−q³.

The subtraction removes exactly the classical cubic slice from
each arm; the incidence bounds are proved in the sibling
`../ghs_n7_membership_20261114/SIDON_BOUND.md`. At q=2 this
gives at least seven nonclassical coefficients per arm; the
earlier native census measured 35 per arm. At q=4 the lower
bound is 1472 per arm. These are **structural eligibility counts**,
not attack-cost bounds or new vulnerability claims.

The symmetric factorization is equivalent to this criterion. Squaring commutes with sigma, so V_i is stable under square and square root. The F2-linear map gamma↦gamma²+gamma has kernel V_i∩F2={0}, since p_i(1)=1; finite injectivity makes it bijective. Thus each nonzero b in V_i has a unique nonzero gamma in V_i with b=[gamma(gamma+1)]². In the generalized GHS presentation gamma has Frobenius order p_i and gamma+1 has order (T+1)p_i. This yields a genus-14 cover. But taking the *same curve* in the classical presentation gamma1=1, gamma2=sqrt(b) gives Frobenius order p_i for sqrt(b), magic number four, and genus 2³−1=7. This is the specialization in Menezes–Teske (2004), Remark 2. The genus-14 presentation therefore cannot supply a lower-genus advantage through its Prym quotient.

Every a with absolute trace one is Artin–Schreier equivalent to a chosen a0 in k with trace one, because [K:k]=7 is odd. The change y↦y+delta*x, with delta²+delta=a+a0, gives the a0 model.

## Explicit F128 D4 model and classical-target identity

Take K=F2[z]/(z⁷+z+1), gamma=z, a0=1, and b=z⁴+z²=0x14. Native Rust checks #E(K)=130=2·5·13. With L(s)=s⁸+s⁴+s and T(s)=L(s)²+L(s), the D4 closure has

    Z: x²+T(s)*x+1=0, v²+v=x+1.

Its automorphisms are A:v↦v+1, B:s↦s+1, and S:x↦1/x,v↦v+L(s). They satisfy B S B=A S. The S-fixed genus-seven quotient is

    X: W²+W=1+1/(L(s)+1).

The translation s↦s+1 sends L(s)+1 to L(s), giving W²+W=1+1/L(s), which is the classical GHS genus-seven equation for this source under the same rational parameter. Thus X is F2-isomorphic to the known target, not merely isogenous.

The native verifier checks #X(F_(2^m))=3,5,9,17,33,65,136 for m=1,...,7, the source order, and the finite closure identities. The separable degree-eight denominator gives genus seven. The point counts and Weil functional equation give L_X(T)=1+T⁷+128T¹⁴ and #J_X(F2)=130.

A separate Sage library replay computed the normalized 112-point divisor correspondence for P=(z⁵+z²+1,z⁶) of order 13. The identity offset is D(O_E)=56([∞0]+[∞1]−2[O_X]), O_X=(s=0,W=0). The corrected F(P)=D(P)−D(O_E) is nonzero and of order 13, and F([7]P)=[7]F(P); target subgroup enumeration returns scalar seven. This is independent preliminary map evidence, pending a committed replay script and native divisor arithmetic. It verifies a known transfer, not a novel attack route.

## Structural exclusion from the symmetric ansatz

For a generalized W1 presentation with gamma2=c*gamma1+d, gamma1∈V_i and c,d∈k*, the original coefficient beta=gamma1*gamma2=c*gamma1²+d*gamma1 lies in V_i. Hence b=beta² also lies in V_i. **Every member of this entire symmetric dihedral ansatz is already classical genus-seven GHS.** Any search for a genuinely new transfer target must leave this ansatz, and a candidate with b outside V_1∪V_2 is an immediate falsifier of it.

This exclusion is a mathematical consequence of Frobenius stability of V_i. It does not exclude other covers or correspondences for the remaining W1 coefficients.

For any separable direct quotient `C→Y` of a genus-fourteen Hess cover,
Riemann--Hurwitz gives `26=deg(C/Y)*(2g(Y)−2)+R` with `R≥0`.
If `g(Y)≥7` and the map is nontrivial, this forces degree two and
`g(Y)=7`, with `R=2`. Thus **any direct quotient retaining target
genus at least seven has this single numerical profile**. The
natural degree-two quotient of the W1 cover has zero norm on the
source odd subgroup, by its anti-invariant action; another
subgroup-preserving genus-seven quotient would need a different
involution or a correspondence outside this direct-quotient class.
This is a cover-degree bound, not an exclusion of all transfer attacks.

## Primary sources and claim labels

- Menezes–Teske (2004), §2 Remark 2 and §5.2, gives the classical genus formula and the published W1 locus: <https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf>.
- Hess (2004), Theorems 12 and 14–16, gives general norm-conorm maps, kernel bounds and cover constraints: <https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf>.
- Characteristic-zero D4 Prym–Jacobian constructions: <https://arxiv.org/pdf/2003.07774>. The characteristic-two polarization case: <https://arxiv.org/pdf/2312.13263>.

Novelty labels: exact curve equation in this study; **known eligible family**; **known genus-seven attack target**; dihedral factorization and explicit map certificate may be a new exposition, subject to a broader prior-art check. No complete cost comparison has been performed for this study.

The repository identity-certificate v1 checker evaluates over the odd
prime `2^61−1`, so it cannot certify these characteristic-two
polynomial identities. Native exhaustive F128 checks and Lean's
limited abstract implications are supplied at their stated scope;
a characteristic-two identity certificate remains open.
