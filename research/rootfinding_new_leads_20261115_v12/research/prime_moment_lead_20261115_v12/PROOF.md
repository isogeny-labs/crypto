# Coherent integer-lift moment certificates for prime-domain boxes

Status: sole-model proof and hand counterexamples; no implementation, independent admission or measurements. This applies to integer interval domains embedded in prime fields with coherent signed coefficient lifts. It does not transfer binary affine linearity, introduce an order on finite fields, or independently resolve modular quotient variables.

Let P be a monic integer quintic with complex roots alpha_l, repeated with multiplicity. Put s_j=sum alpha_l^j, s0=5; Newton identities compute s1..s9 exactly from its integer coefficients. Define five-by-five real symmetric matrices H0[i,j]=s_(i+j), H1[i,j]=s_(i+j+1), i,j0..4.

All roots are real and in[0,M] iff H0, H1 and M*H0-H1 are positive semidefinite. Forward follows from H0=sum v(alpha)v(alpha)^T, H1=sum alpha*v(alpha)v(alpha)^T with positive integer multiplicity weights. Conversely, if a nonreal conjugate pair exists, real interpolation of degree below the number of distinct roots<=5 gives q(alpha)=i,q(conjalpha)=-i and zero at every other distinct root. Then q^T H0 q is negative, contradicting PSD. Once all roots are real, real Lagrange interpolation isolates any negative root or root exceeding M, contradicting H1 or M*H0-H1 respectively. Repeated-root weights remain positive, so degeneracies are included.

This criterion is EXACT for real interval support, but only NECESSARY for integer support. For example X*(X^2-4X+2)^2 has all real roots in[0,4], hence all three matrices PSD, yet its nonzero roots2+-sqrt2 are not integers. A final exact integer-root oracle is still mandatory.

## Exact whole-box rejection

For any fixed rational vector q, every supported P has q^T Hj q>=0 for each of the three matrices. If exact rational interval or Bernstein arithmetic proves an UPPER bound on this expression strictly negative over an entire coherent coefficient box, that box is empty of supported integer polynomials. The retained certificate is the fixed q, matrix choice, coherent box/quotient guards and a replayable exact negative upper bound. A negative midpoint or floating eigenvalue is insufficient. If the bound crosses zero, return UNKNOWN rather than reject. A cheap existing bound should run first; extra moment work must be charged and gated to genuinely unresolved boxes.

The simplest H0 two-by-two determinant is4*e1^2-10*e2>=0. Higher-degree witnesses can add information beyond signed coefficient bounds and that variance test. Consider P=X^5-6X^4+14X^3-16X^2+8X. Its roots are0,2,2,1+i,1-i. At M=2 all elementary coefficients6,14,16,8,0 satisfy the simple interval coefficient bounds, and the variance value is4. Yet s0..s6=(5,6,8,12,24,56,128). For q(X)=X^3-3X^2+2X, coefficient vector(0,2,-3,1,0), q^T H0 q=-8. It therefore fails the exact real-root condition. This hand example motivates a bounded test; it is not measured rejection yield or speed.

## Prime and extension guards

For a finite-field polynomial H, use integer signed coefficients c_j whose reductions equal H and which obey root-derived bounds. Keep every required quotient/lift variable shared across all constraints. Applying the real moment test directly to canonical residues is UNSOUND when elementary coefficients wrap modulo p. If all bounds ensure unique lifts, record the exact uniqueness proof; otherwise branch over coherent quotient cells and retain every possibility. This obligation applies separately at P192/P224/P256/P384/P521. Width is not a hardness certificate.

Odd extensions require a different joint-coordinate lift proof, not an arbitrary real ordering of extension-field elements. Restriction of scalars or Frobenius trace can supply coordinates, but their correlations and modular lifts must remain shared. This lead initially targets prime interval domains only.

Hermite/Newton moment matrices and interval certificates are classical; this is a proposed integration/pruning mechanism, not novelty clearance. Prior local moment_factor_v1 concerns binary Gram decoding, a different use. Existing exact Sturm and Bernstein/lattice methods are mandatory strong specialized comparators.

Next use a preregistered small integer box corpus with planted supported, repeated, nonreal and real-but-noninteger cases. Independently verify every negative certificate by rational polynomial/interval expansion and compare with exact real-root isolation. Test wrapped finite-field examples retaining all coherent lifts. Only after correctness, freeze held-out boxes across all five NIST primes and compare net whole-box exclusions and TOTAL costs against the unchanged existing bounds/Sturm pipeline, including lift preparation, witness discovery, certificate replay and fallbacks. Initial90s/4GiB/onecore, no statistical or stage-only speed claims.
