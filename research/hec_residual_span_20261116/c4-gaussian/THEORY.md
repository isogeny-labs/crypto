# Conditional Gaussian reconstruction of C4 scalar action

This is a proposed setup mechanism, not a tenfold solve theorem or novelty claim. Let an ordinary genus4curve have odd f of degree9 over Fp,p=1mod4, and sigma(x,y)=(-x,i*y). Exact N=2ell is supplied, ell oddprime. No source/target/fullWeilpolynomial/eigenvalue is passed to the reconstruction.

## Oriented trace from an Fp character sum

Choose nonsquare d and zeta²=d in Fp², so zeta^p=-zeta. Put H(t)=f(zeta*t)/zeta in Fp[t]; for odd f this is t*(d4*t8+a7*d3*t6+a5*d2*t4+a3*d*t2+a1). Let kappa=d^((p-1)/4), one of±i, and epsilon=(-i)/kappa in{±1}. The affine fixedpoints of sigma*pi have x=zeta*t and y^p/y=-i. For H(t)!=0 there are two fixed y precisely when kappa*chi(H(t))=-i; H=0 contributes one. Infinity contributes one. Therefore

 C_twist=1+p+epsilon*sum_t chi(H(t)),
 Tr(sigma*pi)=-epsilon*sum_t chi(H(t)).

On the sigma=i eigenspace write the degree4Frobenius polynomial G(T)=T4+aT3+bT²+cT+d0 over Z[i]. The conjugate eigenspace has barG, so ordinary pointcount gives 2Re(a)=C1-p-1 and twistedtrace gives 2Im(a)=Tr(sigma*pi). Both are computed with a square-character table and paired t,-t scans; no extension enumeration or large scalar multiplication is used.

## Determinant hypothesis and finite candidate construction

HYPOTHESIS Hdet: d0=u*p² for a Gaussian unit u. Reciprocity then forces c=u*p*bar(a), b=u*bar(b). The bound |b|≤6p follows from four Frobenius roots of magnitude sqrt(p). The natural proof route for Hdet is that sigma has balanced2+2Hodge signature (basis x^j dx/y,j0..3 has alternating i,-i) and ordinarity makes each Gaussian Frobenius determinant have p-adic valuations2at both primes overp. A fully sourced/independently checked determinant lemma is still required for an unrestricted algorithmic claim; it is NOT asserted proved by this note. On retained controls the entire exact equality G*barG=F and original-group scalar checks provide additional certificates of the actual reconstructed factor without relying on Hdet as a generic theorem.

Write Z_u=1+a+u*p*bar(a)+u*p². Then G(1)=A+B*i=Z_u+b and A²+B²=N. For u=1,b isreal; u=-1,b ispureimaginary; u=i,b=t(1+i); u=-i,b=t(1-i). Intersect the integer circle normN with each corresponding line, retain every integral intersection satisfying |b|≤6p, and deduplicate. At most8candidates occur. Missing/multiple candidates remain explicit; do not pick by expectedlambda or sourceaction. Under Hdet the actual G must be among them. If exactly one remains it is identified without extra extensioncounts or suppliedF.

For u=i, e.g., D=Z_real-Z_imag, solve (A+B)²=2N-D². Equivalent t=(-Z_real-Z_imag±sqrt(2N-D²))/2. u=-i uses D=Z_real+Z_imag and the other diagonal; real/imaginary lines use one coordinate fixed. All arithmetic integral with exactsquaretests, no floating approximations.

## From the oriented norm to scalar action

F=G*barG onTate modules. Replacing formal i by sigma yields the operator identity G(pi,sigma)=0, componentwise on sigma's two eigenspaces. On rational oddell-torsion pi=1, hence (A+B*sigma)D=0. B cannot vanishmodell: then normN=2ell implies A=0modell, contradicting ellvaluation1. Thus lambda=-A/B modell, with lambda²=-1. Orientation is bound by the geometrically specified i and the sigma*pi fixedpoint count, not an arbitrary Gaussianfactor sign. Both finite-field i choices must give conjugate factors and opposite lambda.

## Prior art and boundaries

Primary Park–Jeong–Lim, EUROCRYPT2002, section3Example2 already gives this odd-polynomial C4automorphism; section4.1 uses its eigenvalue for scalar decomposition. Author-uploaded fulltext read at https://www.researchgate.net/publication/221348702_Speeding_Up_Point_Multiplication_on_Hyperelliptic_Curves_with_Efficiently-Computable_Endomorphisms . IACR PDF URL returned403/unavailable. Frobenius/Cayley-Hamilton/Lefschetz/norm arithmetic are classical; this combination is a candidate adaptation with no priority claim. KB queries for Gaussian/twistedtrace/eigenspacedeterminants returned no load-bearing exactconstruction; irrelevant broad hits are not proof of absence. Revision9b2119e8fbac482b98b50521c0d604bb, lexical. Existing exact source/order/Weil records and C4phasecosts inform this bounded gate.

Even a large eigenvalue-stage reduction is limited by that stage's18.8–25.3%baseREDCshare in the present pencil. This cannot alone establish the broad10xobjective. The point is a mathematically distinct way to remove costly group-based setup, to compose only if its reconstruction/cost gates pass.
