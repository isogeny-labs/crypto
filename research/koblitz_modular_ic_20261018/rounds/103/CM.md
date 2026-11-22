# R103: direct lambda-derivative norm on the degree-112 source

The actual R51 polynomial has additive lambda shape

    G51 = l^16 + A(x,y)l^8 + B(x,y)l^4 + C(x,y)l + D(x,y).

R101 checked the complete lambda exponent grid in the 29 source terms.
In characteristic two, `dG51/dl=C`. Because `G51` is monic degree 16
and C is l-independent, `Res_l(G51,C)=C^16`: over a splitting field this
is the product of the same C over all 16 roots, with multiplicity. Taking
the norm through monic degree-7 f51 gives

    Res_y(f51, Res_l(G51,dG51/dl)) = Res_y(f51,C)^16.

R103 computed the right-hand direct norm from the actual donor:

    Res_y(f51,C) = P123(x)/x^163,

where `P123` is byte-for-byte the degree-123 R100 Frobenius root. Its
sixteenth power equals R99's full norm `P123^16/x^2608` in F2(x).
The source-bound native Singular process exited zero, had no `?` errors,
checked a changed-norm negative control, and printed the exact pass
marker. A fresh process reproduced `direct-norm103.sing` byte-for-byte.
The independently committed `verify103.py` checked the three source
hashes, both numerator exponent sets, denominators, and degrees. Lean
4.34.1 checks the exponent parity and degree/denominator arithmetic;
the general resultant identity above is a mathematical proof and native
source identity, not an authored Lean theorem.

This direct computation is a smaller exact route to the finite support
of the displayed lambda monogenic discriminant. It does not give
normalized ramification. Its next use is to evaluate C on the local
normalizations at x=1 and the degree-11/22 supports, with R97's
degree-7 base support and x=0/infinity handled separately.

The raw archive containing the producer and fresh replay has SHA-256
`1136fb2502eae1d22a4d5b9604e01bd2163d9438725ec7e6d19891c3c1b49e4d`.
The report PDF is generated from the exact result with a dependency-free
renderer; PDF structure and recorded values are checked. This host lacks
PDF rasterization, so no visual raster inspection is claimed.

Sources: R51 donor, R99 norm, R100 factor root, R101 exponent grid;
[Stacks resultant determinant](https://stacks.math.columbia.edu/tag/00UA)
and [Stacks norm map](https://stacks.math.columbia.edu/tag/0BD3).
