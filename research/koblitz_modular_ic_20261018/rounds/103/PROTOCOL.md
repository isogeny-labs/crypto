# R103: direct lambda-derivative norm from the actual degree-112 source

The R101 source audit found only lambda exponents 16, 8, 4, 1, 0 in the
actual monic degree-16 G51. In characteristic two, its derivative in
lambda is the l-linear coefficient C(x,y). For any monic degree-16 G,
`Res_l(G,C)=C^16` when C is lambda-independent. The norm through the
actual monic degree-7 f51 therefore has sixteenth power
`Res_y(f51,C)^16`. Compute this smaller direct norm in pinned Singular,
check that it is nonzero and in F2(x), and compare its sixteenth power
against the exact R99 full resultant norm. Reject a changed equality.
Export direct numerator and denominator, then fresh replay against the
source-bound R51 donor and R99 norm output.

One local pinned Singular process, seed103, one thread, 2 GiB virtual
memory, 120-second timeout. Commit this source/protocol before execution.
Bind the R51 donor SHA-256 and R99 norm output SHA-256 in the raw driver;
retain all failed attempts. This is exact algebra with no timing/statistical
comparison. The default EKS image lacks the pinned CAS binary.

Sources: R51 triangular donor, R63 source field, R99 exact norm, R100
Frobenius-root factors, R101 additive exponent grid;
[Stacks resultant determinant example](https://stacks.math.columbia.edu/tag/00UA)
and [Stacks norm map](https://stacks.math.columbia.edu/tag/0BD3).
