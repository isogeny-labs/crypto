# Centroid incidence exposes an affine-linearized quartic

Status: new program lead with a self-contained sole-model derivation and hand-checkable controls. No new worker, independent admission, scientific execution, novelty clearance or timing exists. Keep every original branch.

Let K be any finite field of characteristic two and H=X^5+bX^4+e2X^3+e3X^2+e4X+e5. Consider a supported source multiset of five elements, so b is their sum. Affine rank means rank of source differences over F2, not over K.

## Exact frontier for five distinct sources

Five distinct points have affine rank at most3 iff b is one of them. An affine dependence over F2 is an even nonempty subset summing to zero. Among five distinct points its size cannot be2, so it must be4. The omitted fifth point then equals the total sum b. Conversely a source equal to b leaves four distinct points with zero sum, giving such an affine dependence. Five distinct points cannot have affine rank below3, because a two-dimensional binary affine plane contains only4 points. Thus five-distinct affine rank3 iff H(b)=0, and rank4 iff H(b)!=0. This requires distinctness. Every repeated five-multiset has at most4 distinct points, hence affine rank<=3 whether or not H(b)=0.

Let D=V^2+e2*U*V+e4*U^2 with U=e3+b*e2, V=e5+b*e4, the previously proved inversion-free quintic resultant. H is squarefree iff D!=0. The complete supported-source partition is therefore: repeated(D=0); squarefree/rank3(D!=0,H(b)=0); squarefree/rank4(D!=0,H(b)!=0). This is a guarded source-pattern classification, not a splitting test for arbitrary H. A nonsplit H can satisfy the last two scalar guards.

## Exact quartic consequence, including repeated roots

On H(b)=0, exact division by X+b gives

    Q=X^4+uX^2+vX+w,
    u=e2, v=e3+b*e2, w=e4+b*e3+b^2*e2.

The cubic coefficient vanishes because e1(H)=b. The remaining equation e5=b*w is precisely H(b)=0. Thus T(x)=x^4+u*x^2+v*x is F2-linear. For an admitted affine source domain a+span(d_i), solve

    sum t_i*T(d_i)=w+T(a).

Binary Gaussian elimination returns ALL domain roots as a particular plus an independent kernel. This is one linear solve for a supplied Q, not enumeration of2^d domain elements. The number of distinct roots is at most4. If v!=0 each algebraic root has multiplicity1, so complete support requires exactly4 domain roots. If v=0,u!=0, Q is the square of a separable quadratic over the perfect field K; require exactly2 domain roots, each with multiplicity2. If v=u=0, Q=(X+fourthroot(w))^4; require exactly1 domain root with multiplicity4. Empty or smaller sets reject full support; larger sets contradict the monic quartic and are an error. Add b with one more multiplicity, merging it if already among Q's roots. Enumerate all distinct five-source orderings only after this exact recovery. Point signs/original auxiliary fibers remain separate.

On this cell the old resultant specializes to

    V=b^2*v, D=v^2*Q(b), Q(b)=b^4+b^2*u+e4.

Thus the two possible repeated mechanisms are explicit: v=0 or b also roots Q. No division or squarefree assumption is needed for the linear root solver.

## Coupling to the actual norm family

At fixed h,k in the existing binary norm chart, every H coefficient is F2-affine in the SAME r/s coordinates. b=h^2+h+z is fixed and admitted in the source domain. Therefore H(b)=e5+b*e4+b^2*e3+b^3*e2=0 is one83-bit affine condition on the known73-bit relaxation. It can be solved directly as an83-by73 binary system; its rank, consistency and surviving dimension must be measured. This is a known candidate root b, not a fresh2^18 scan over an unknown source root. On each surviving H, recover Q by the exact linear formulas above and use the affine-linearized root solver. No generic rank bound or small survivor count is assumed. A large surviving family is UNKNOWN under a declared cap.

This potentially gives a cheaper COMPLETE conditional five-distinct/rank3 norm branch than an all-source incidence scan. It leaves the squarefree/rank4 branch, other repeated cases, outer h/k choices and original all-root closure unresolved. Existing scalar incidence and low-rank moment work are strong comparators and conceptual overlap; the observation is not a new linearized-factorization principle.

## Independent hand controls and a necessary counterexample

Over GF8 with t^3=t+1, roots{0,1,2,4,7} sum to b=0 and have affine rank3. Dividing by X gives Q=X^4+6X^2+5X+2. Direct multiplication of (X^2+3X+2)(X^2+3X+1) gives these coefficients. Its four roots are1+span{3,5}={1,2,4,7}; T(1)=T(2)=T(4)=T(7)=2, v=5!=0 and Q(0)=2!=0.

Over GF16 the five roots{0,1,2,4,8} have independent differences1,2,4,8, sum15 outside the set and H(15)!=0. Conversely{0,0,1,2,4} has sum7 outside the set but affine rank3. This is a concrete counterexample to using H(b)!=0 alone as a rank4 test: the squarefree guard is mandatory. These are hand derivations, not reported executable oracle results.

Next exhaust all five-root multisets over GF4/GF8/GF16:56+792+15504=16352 cases. Independently compare direct source rank, root multiplicities, scalar guards, literal division and the complete linearized-domain solution sets, including empty/nonprefix/affine domains. Then freeze the actual h,k0 family, independently reconstruct the centroid-incidence system and compare its exact family with the unchanged literal incidence solver at x=b. Compare TOTAL cost against both that strong known-root comparator and any full-domain incidence chart, explicitly labeling their different contracts. Before timing require independent source review and hash/budget freeze.
