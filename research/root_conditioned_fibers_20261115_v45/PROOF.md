# Root-conditioned affine norm fibers

At fixed outer h (and k=0), the normalized binary quintic coefficients are affine F2-linear in u,v. Restricting to coefficient-span constraints gives an exact affine fiber p+span(B), of dimension m (73 in the recorded actual charts). Write its polynomial as H_t=H_0+sum t_j C_j. For any declared candidate source root x, H_t(x)=0 is the exact linear system sum t_j C_j(x)=H_0(x) over the n-dimensional binary coordinate space.

Gaussian elimination returns either an empty whole cell or every parameter as an affine fiber of dimension m-rank. No generic rank assumption is needed. Every supported monic quintic has a root in V, so the union over x in V covers every supported polynomial. Disjoint coverage can use the least root in the declared canonical field-element order. Repeated roots, x=0 and degenerate linear maps stay in the union. A bounded implementation must return unresolved when a conditioned fiber exceeds its enumeration/output cap.

This changes a direct 2^m candidate enumeration to 2^d linear systems plus their compatible residual fibers per h, if ranks are high. It does not remove the 2^d outer h cover, and field arithmetic/elimination/support/output must all be charged. Root conditioning and linear algebra are standard; novelty of this specialized combination is not established. Original-chain intermediate-coordinate and exceptional-chart coverage is not supplied by a norm-family result.

The experiment tests complete small affine fibers against literal u,v enumeration and literal root-product support dictionaries, then profiles frozen actual h,x cells. Rank deficiency is evidence against the proposed cost reduction, not an empty-root conclusion.
