# Actual degree-three cover and the chained Jacobian embedding

Written proof candidate, prepared for the second claim record. Native coefficient and finite-control inputs are the whole source-C certificate and unsigned PCC-20261127-758cbf. This document separates those inputs from the standard geometric deductions below; it is not a full Lean or independently signed proof. [pcc:unverified]

## Objects and full coefficient certificate

Work over k=F7 with the smooth proper models of

    D: Y^2=H(W)=6W^6+2W^4+1,
    E: v^2=u^3+2u+6.

Use F=W^3+W^2+3W+1, G=6W^3+W^2+4W+1,
N=W^3+5W^2+W+3 and R=2W^3+W+2. The complete native certificate records

    H=FG,
    N^3+2NG^2+6G^3=FR^2,
    N'G-NG'=(3W+5)R.

Its actual Bezout pairs (s,t) for sA+tG=1 are:
N: ([3,0,6],[6,1,6]); R: ([2,1,3],[4,1,6]);
F: ([4,5,2],[4,2,2]), in ascending coefficients. Both cubics F,G have no k-root and hence are irreducible. The old squarefree certificate is

    (1+W^4)H+(4W+4W^3+W^5)H'=1.

The short-Weierstrass discriminant of E is 1 in k. Thus D is a genus-two smooth proper hyperelliptic curve and E is nonsingular. These are the original models and coefficients, with no replacement of the target subgroup or field. [pcc:unverified]

## A genuine separable degree-three morphism

Set H3=(N/G,YR/G^2). The displayed full identity proves the target equation in k(D), so these rational functions define a nonconstant map to E. Coprime N,G and their degree three imply [k(W):k(N/G)]=3. The hyperelliptic extension has degree two, and [k(E):k(u)]=2. The embedded target function field therefore has index three in k(D). Its inseparable degree is a power of seven dividing three, hence is one. The rational map uniquely extends to a finite degree-three morphism of the smooth proper curves. No interpolation or finite-point test proves this step. [pcc:unverified]

At a root a of G, H squarefree and F(a) nonzero make Y a local parameter, with G=Y^2/F. Consequently u has pole order two, v has pole order three, and

    -u/v=-NY/(FR)

has order one and a nonzero leading coefficient. Thus the three G-Weierstrass points map unramifiedly to O and form its complete degree-three fibre. The named Bezout witnesses guarantee all the required units. [pcc:unverified]

At infinity put t=1/W and eta=Y/W^3. The two conjugate branches have eta^2=6, and direct leading coefficients give H3(infinity)=(6,2eta). The two limits are opposite elliptic points; their sum is O. The source-C quotient-field calculation checks both limits, their target equations and their sum. These are proper extensions, including the nonsplit branches. [pcc:unverified]

The invariant differential pulls back as

    H3^*(du/(2v))=((3W+5)/2) dW/Y.

The canonical divisor of dW/Y is Dinf=infinity+ + infinity-. The factor (3W+5)/2 has a simple pole at each infinity and zeros at W=3. Hence the ramification divisor consists exactly of (3,1) and (3,-1), each with index two; the branch values are (4,6) and (4,1). This also gives the full Riemann--Hurwitz ramification degree two. [pcc:unverified]

At P0=(0,1), H3(P0)=(3,2). Translation by (3,5) gives a based map h3(P0)=O. Translation acts trivially on Picard degree-zero classes, so H3 and h3 induce the same Jacobian pullback. [pcc:unverified]

## Identify the actual chained map globally

Let p1=(3W^2+5,Y), p2=(1/W^2,Y/W^3) be the original proper degree-two covers, and pi the actual seventh-power Frobenius morphism of E. Put w=pi+2 and barw=1-w. Source C uses Frobenius on the function field, including its action on W and Y, to certify the full identity

    [2]H3=p1+w composed with p2.

Passing to Albanese maps and then dual Picard maps gives

    [2]H3_* = p1_*+w p2_*,
    [2]H3^* = p1^*+p2^* barw.

The complete rational point count is eleven, giving Frobenius trace -3. Dual Frobenius is Verschiebung and pi+barpi=-3, so the dual of w is barw. The original two involutions give p1_*p2^*=p2_*p1^*=0, while push--pull for either degree-two cover is [2]. These are identities of algebraic homomorphisms. [pcc:unverified]

Hom groups of abelian varieties are torsion-free: multiplication by a nonzero integer is surjective on the domain, so n f=0 forces f=0. Cancelling two therefore yields

    p1_*H3^*=I,    p2_*H3^*=barw.

The original actual Prym construction transported the chained beta3 map to the genuine integral Gamma satisfying 2Gamma=p1^*+p2^*barw. The same cancellation now proves H3^*=Gamma globally. This identifies the explicit cover with the actual chained image, rather than merely agreeing modulo eleven. Its principal norm is H3_*H3^*=[3]. The global p1 retraction makes Gamma injective on every subgroup scheme; no inversion of three or restriction to odd-order points is required for this conclusion. [pcc:unverified]

## Coordinate-only canonical divisor evaluation

Write DG=H3^*(O), with ideal (monic G,0). For each finite Q=(x,y) in E(k), source C constructs the actual effective fibre

    UQ=monic(N-xG),   VQ=yG^2/R mod UQ.

There is no finite k-point with x=6 or y=0, so the cubic leading coefficient and the needed inverse of R are nonzero. The canonical class is [DQ-DG]. Since div(G)=2DG-3Dinf, this is also [DQ+DG-3Dinf]. Thus exact affine Cantor composition of the two cubic fibres followed by balanced reduction produces the genuine Jacobian class. Repeated roots are retained as ideal multiplicities. At the two branch values UQ=(W-3)^2(W-5), and the native VQ satisfies the full doubled-root congruence. [pcc:unverified]

For an even semireduced degree 2n representative, Y-V has the same pole order m=max(3,deg V) at both infinities. When deg V=3, cancellation would require a square leading coefficient equal to the nonsquare 6, which is impossible. Its complementary divisor therefore has degree 2m-2n, and reflection gives the same class with equal infinity counterweights. The source-C reduction from degrees six/four to two/zero is the established balanced Cantor method specialized to this inert model. Riemann--Roch and the canonical Dinf give the unique nonzero quadratic representative, with identity (1,0), as proved in the original arithmetic component. [pcc:unverified]

The API receives only the specified curve, generator and target coordinates. On all eleven k-inputs the resulting ideals equal the original Gamma recipe and their actual pushforwards are (Q,-2Q), with H3_*Gamma(Q)=3Q. All 121 original classes are pushed by evaluating real roots and multiplicities in k or their quadratic residue fields; irreducibility of G excludes poles in those supports, and H3_*Dinf=O. These are complete finite checks in addition to the global proof. [pcc:unverified]

## Preserved scope and proof obligations

Replacing geometric pi by the identity retains all fourteen rational-point values but fails the full rational-function identity; both residuals are published. The actual N0+1 and R0+1 controls also fail and restore. These negatives explain why finite tables alone cannot identify the global chain. [pcc:unverified]

This applies established degree-three-cover, divisor and Jacobian theory to the actual F7 model. Shaska's reviewed characteristic-zero classification is architectural prior work, not a theorem imported over F7. F49/F343 describe auxiliary residue geometry; the transfer and target subgroup remain over F7. Full generic Lean definitions for these proper curves, their Picard/Jacobian maps and the polarization/chain identification remain open. The formal proof map records the separately blocked current compiler/cache setup and makes no new accepted-Lean claim.
