# Projected Euclidean resultant

Let p be caller-certified prime, p-1=q*t, q in {2,4}, t odd, and pi(z)=z^t. For nonzero z, pi(z) has order dividing q and pi(-1)=-1. Define pi(0)=0. Polynomial coefficients and all divisions remain in F_p.

For A,B of degrees m,n with nonzero remainder R=A mod B of degree k, the standard resultant identity is

    Res(A,B)=(-1)^(mn) lc(B)^(m-k) Res(B,R).

For n=0, Res(A,B)=B^m; if R=0 with n>0 the resultant is zero. For m<n swap A,B and accumulate (-1)^(mn) without division. The implementation accumulates the parity sign and multiplies lc(B) only (m-k) mod q times, so each transition contributes at most three coefficient multiplications. At termination it raises the accumulated nonzero representative to t and applies the accumulated sign. This is valid because pi(lc(B)^e)=pi(lc(B)^(e mod q)). Reduction of exponents is valid only in the multiplicative quotient: no coefficient addition is projected.

For F(X)=product_i(X-a_i) and G(X)=product_j(X-b_j), the old root-product convention is product_j F(b_j)=Res(G,F), not Res(F,G). The wrapper deliberately reverses the polynomial arguments. Repeated roots and shared roots are permitted and yield the corresponding zero resultant. Balanced root_product recursively retains only coefficient vectors, not Tree children; the geometry uses projected main and tail resultants and multiplies them directly.

## Costs and boundary

This is ordinary Euclidean division, not half-GCD. It may perform many divisions, including inversions of nonmonic leading coefficients. Each leading-coefficient exponent is reduced below q; each resultant receives one terminal field projection. A nonempty geometry tail therefore receives a second projection, unlike the preceding backend's one projection after multiplying full resultants. No timing, operation-total or peak-memory advantage has been measured. Keep the previous backend as the reference/candidate for comparison.

The larger scaling audit matters: h=(r-1)/2=2wk+2s+e, with 0<=s<w and e in {0,1}. Thus h+1<=2w(k+1), and (w+k+1)^2>=4w(k+1)>=r+1. The current generic geometry loops explicitly visit w+k entries, at least ceil(sqrt(r+1))-1. This is a bound on this implementation's point-list representation, not on arbitrary ECDLP algorithms. The current quartic compiler additionally enumerates all r scalar-field inputs to form signature fibers and allocates dense coefficient systems. Neither cost disappears through a resultant backend change. The existing fixed affine-linear decoder lower bound is already proved in feature-native-linear-constructor/DERIVATION.md; it does not rule out nonlinear or adaptive constructions.

Next: compose Euclidean transition representatives before terminal projection, investigate half-GCD batching with exact degree-drop/sign accounting, and pursue an implicit orbit-product representation that avoids enumerating w+k geometry points. Such a representation needs a cheap evaluator, not just a short circuit description. Decoder construction remains a separate structural dependency.

## Sources and scope

Read feature-native-resultant-product, feature-native-monomial-recovery, feature-native-quartic-collapse and feature-native-linear-constructor. FLINT's primary documentation gives the Euclidean resultant identity and distinguishes ordinary Euclid from half-GCD: https://flintlib.org/doc/algorithms/polynomials/gcd.html . The recurrence is standard; no novelty-priority claim. KB unavailable: no loaded search tools and scripts/research-kb status reports no executable. Only prime-field order-two/order-four projections are supported; F17 is an explicit rejected control. No characteristic transfer, index calculus, GHS or collision recovery is introduced.
