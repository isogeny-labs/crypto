# Orbit incidence norm and obstruction to scalar descent

[pcc:unverified] Mathematical derivation for review, not independently certified. This is a screen of the next collection mechanism following lazy-certificate-cost, not a speed result.

Let k have odd characteristic not dividing n=2g+1, let z have exact order n, and let the smooth curve be y²=x^n+c. Rotation sigma sends (x,y) to (zx,y). For a monic anchor polynomial u of degree g and v of degree <g, the pencil function is tau=(y-v(x))/u(x).

## The pencil does not descend to rotation invariants

If tau(zx,y)=tau(x,y) in k(C), then

    y [u(x)-u(zx)] = v(zx)u(x)-v(x)u(zx).

Since y is not in k(x) on a smooth hyperelliptic curve, 1 and y are linearly independent over k(x). Hence u(zx)=u(x). Comparing the leading coefficient forces z^g=1, impossible because 0<g<n and z has order n. Thus the individual pencil parameter is not a rational function of the orbit invariants (x^n,y). More generally sigma^d cannot stabilize tau for 0<d<n: it would require n | dg, but gcd(g,2g+1)=1. The stabilizer is trivial even at n=9. Consequently tau has degree n over the invariant function field, by the Galois orbit-stabilizer theorem. This is a generic function-field statement, not a claim that every finite rational fiber has n distinct values.

This covers g=2,3,4 binomial models when the rational rotation exists, with no prime-n assumption. It says nothing about general curve models or a lower bound on all relation algorithms.

## Explicit genus-two norm

For r=x0^5 nonzero, define

    N_r(A,B,C) = C^5 + r(B^5 - 5 A C B^3 + 5 A² C² B) + A^5 r².

Then N_r(A,B,C)= product over alpha^5=r of (A alpha²+B alpha+C).
Proof: if A != 0, let beta,gamma be the quadratic roots, with sum s=-B/A and product h=C/A. Newton recurrence gives beta^5+gamma^5=s^5-5hs^3+5h²s. The product equals A^5[(beta gamma)^5-r(beta^5+gamma^5)+r²], yielding the formula. Polynomial identity extends to A=0, including B=0. No characteristic-zero division appears in the final formula.

For u=X²+u1 X+u0, v=v1 X+v0 and signed orbit ordinate epsilon*y0, set

    A=T, B=v1+u1 T, C=v0+u0 T-epsilon*y0.

The resulting degree-at-most-five polynomial is exactly product_e [v(z^e x0)+T u(z^e x0)-epsilon*y0]. Its leading coefficient is product_e u(z^e x0). Both signs give degree at most ten. Away from denominator zeros its roots, with multiplicity, are precisely the original explicit pencil parameters. Root collisions must be retained, not counted as independent relations.

If an orbit meets u=0 and v=epsilon*y0 there, a product factor is identically zero: the norm is identically zero and cannot act as a useful parameter filter. This is a base-point issue, not evidence that every parameter produces a residual support point. Remove those fixed factors explicitly, or saturate before elimination; a plain norm-zero filter is unsound there. If u=0 but v differs, the constant factor is nonzero and can remain.

## Consequence for the proposed mechanism

Replacing orbit expansion by one scalar parameter is invalid. The norm is a sound implicit representation after base-point handling, but solving/gcd/root-lifting, multiplicities, signs and row weights still cost work. Degree n in the invariant function field prevents claiming a n-fold rational descent of this pencil; it does not prevent useful batched polynomial algorithms. A successor would need an explicit shared incidence algorithm and complete cold cost case before a performance experiment. Do not infer 10x from compression of orbit storage.

## Sources checked

KB revision 9b2119e8fbac482b98b50521c0d604bb, last_refresh 2026-11-17T06:15:25Z, ready true, lexical current/hybrid absent. It predates this session's results, so current local source and raw records are authoritative. Queries: hyperelliptic pencil orbit invariant resultant automorphism relation collection; orbit invariant incidence index calculus negative scan compression; pencil symbolic genus2. source:91b6bbe9fb73267bbeb5d7c97f493b26 (crypto:docs/ecbench/README.md) identifies existing symbolic-pencil methods and incomplete legacy accounting, not this mechanism's proof or cost. source:fdff62973a741275cdbf8ec3b5ff069d (crypto:docs/ic/LEAD_LADDER_METHODOLOGY_20261010.md, L4 excerpt read) distinguishes orbit folding from algebraic membership. No novelty claim.

Primary reference for the standard resultant product convention: https://doc.sagemath.org/html/en/reference/polynomial_rings/sage/rings/polynomial/polynomial_rational_flint.html#sage.rings.polynomial.polynomial_rational_flint.Polynomial_rational_flint.resultant (read). The formula and descent argument above are the present specialization; this reference does not establish a speedup. Local inputs: lazy-certificate-cost/RESULTS.md; lazy-certificate/core.rs.
